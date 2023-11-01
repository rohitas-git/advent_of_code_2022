use camino::Utf8PathBuf;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::{bytes::complete::take_while1, combinator::map, IResult};
use std::collections::HashMap;

mod parsing_input {
    use super::*;

    fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
        map(
            take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
            Into::into,
        )(i)
    }

    #[derive(Debug)]
    pub struct Ls;

    fn parse_ls(i: &str) -> IResult<&str, Ls> {
        map(tag("ls"), |_| Ls)(i)
    }

    #[derive(Debug)]
    pub struct Cd(Utf8PathBuf);

    fn parse_cd(i: &str) -> IResult<&str, Cd> {
        map(preceded(tag("cd "), parse_path), Cd)(i)
    }

    #[derive(Debug)]
    pub enum Command {
        Ls,
        Cd(Utf8PathBuf),
    }

    impl From<Ls> for Command {
        fn from(_ls: Ls) -> Self {
            Command::Ls
        }
    }

    impl From<Cd> for Command {
        fn from(cd: Cd) -> Self {
            Command::Cd(cd.0)
        }
    }

    fn parse_command(i: &str) -> IResult<&str, Command> {
        let (i, _) = tag("$ ")(i)?;
        alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
    }

    #[derive(Debug)]
    pub enum Entry {
        Dir(Utf8PathBuf),
        File(u64, Utf8PathBuf),
    }

    fn parse_entry(i: &str) -> IResult<&str, Entry> {
        let parse_file = map(
            separated_pair(nom::character::complete::u64, tag(" "), parse_path),
            |(size, path)| Entry::File(size, path),
        );
        let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

        alt((parse_file, parse_dir))(i)
    }

    #[derive(Debug)]
    pub enum Line {
        Command(Command),
        Entry(Entry),
    }

    pub fn parse_line(i: &str) -> IResult<&str, Line> {
        alt((
            map(parse_command, Line::Command),
            map(parse_entry, Line::Entry),
        ))(i)
    }

    fn parse_inputs() {
        use camino::Utf8PathBuf;
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_while1},
            combinator::{all_consuming, map},
            sequence::{preceded, separated_pair},
            Finish, IResult,
        };

        fn main() {
            let lines = include_str!("../demo.txt")
                .lines()
                .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

            for line in lines {
                println!("{line:?}");
            }
        }

        main()
    }

    // #[cfg(test)]
    // mod test {
    //     use super::*;

    //     #[test]
    //     fn test_solution() {
    //         solution()
    //     }
    // }
}

mod naive_tree {
    use super::*;
    use indexmap::IndexMap;
    use std::cell::RefCell;
    use std::collections::BTreeMap; // to keep keys sorted!
    use std::rc::Rc; // to retain insertion order!

    type NodeHandle = Rc<RefCell<Node>>;

    #[derive(Default)]
    struct Node {
        size: usize,
        children: IndexMap<Utf8PathBuf, NodeHandle>,
        parent: Option<NodeHandle>,
    }
    use std::fmt;
    // impl std::fmt::Debug for Node {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         f.debug_struct("Node")
    //             .field("size", &self.size)
    //             .field("children", &self.children)
    //             .finish()
    //     }
    // }

    // newtype
    struct PrettyNode<'a>(&'a NodeHandle);

    impl<'a> fmt::Debug for PrettyNode<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let this = self.0.borrow();
            if this.size == 0 {
                writeln!(f, "(dir)")?;
            } else {
                writeln!(f, "(file, size={})", this.size)?;
            }

            for (name, child) in &this.children {
                // not very efficient at all, but shrug
                for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                    if index == 0 {
                        writeln!(f, "{name} {line}")?;
                    } else {
                        writeln!(f, "  {line}")?;
                    }
                }
            }
            Ok(())
        }
    }

    impl Node {
        fn is_dir(&self) -> bool {
            self.size == 0 && !self.children.is_empty()
        }

        fn total_size(&self) -> u64 {
            self.children
                .values()
                .map(|child| child.borrow().total_size())
                .sum::<u64>()
                + self.size as u64
        }
    }

    fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
        // clippy is wrong and should feel bad
        #[allow(clippy::needless_collect)]
        let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

        Box::new(
            std::iter::once(n).chain(
                children
                    .into_iter()
                    .filter_map(|c| {
                        if c.borrow().is_dir() {
                            Some(all_dirs(c))
                        } else {
                            None
                        }
                    })
                    .flatten(),
            ),
        )
    }

    #[test]
    fn test_solution() {
        use super::parsing_input::*;
        use camino::Utf8PathBuf;
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_while1},
            combinator::{all_consuming, map},
            sequence::{preceded, separated_pair},
            Finish, IResult,
        };

        fn main() {
            let lines = include_str!("../input.txt")
                .lines()
                .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

            let root = Rc::new(RefCell::new(Node::default()));
            let mut node = root.clone();

            for line in lines {
                println!("{line:?}");
                match line {
                    Line::Command(cmd) => match cmd {
                        Command::Ls => {
                            // just ignore those
                        }
                        Command::Cd(path) => match path.as_str() {
                            "/" => {
                                // ignore, we're already there
                            }
                            ".." => {
                                let parent = node.borrow().parent.clone().unwrap();
                                node = parent;
                            }
                            _ => {
                                let child =
                                    node.borrow_mut().children.entry(path).or_default().clone();
                                node = child;
                            }
                        },
                    },
                    Line::Entry(entry) => match entry {
                        Entry::Dir(dir) => {
                            let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                            entry.borrow_mut().parent = Some(node.clone());
                        }
                        Entry::File(size, file) => {
                            let entry = node.borrow_mut().children.entry(file).or_default().clone();
                            entry.borrow_mut().size = size as usize;
                            entry.borrow_mut().parent = Some(node.clone());
                        }
                    },
                }
            }
            // println!("{root:#?}");

            let sum = all_dirs(root.clone())
                .map(|d| d.borrow().total_size())
                .filter(|&s| s <= 100_000)
                .inspect(|s| {
                    dbg!(s);
                })
                .sum::<u64>();
            dbg!(sum);

            let total_space = 70000000_u64;
            let used_space = root.borrow().total_size();
            let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
            let needed_free_space = 30000000_u64;
            let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

            let removed_dir_size = all_dirs(root)
                .map(|d| d.borrow().total_size())
                .filter(|&s| s >= minimum_space_to_free)
                .inspect(|s| {
                    dbg!(s);
                })
                .min();
            dbg!(removed_dir_size);
        }

        main()
    }
}
