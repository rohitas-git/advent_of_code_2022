use std::{
    array,
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Display,
    ops::Deref,
    process::Command,
    rc::{Rc, Weak},
};

pub use super::utils::Cmd::*;
pub use super::utils::Entry::*;
use super::utils::*;
pub use GoTo::*;
pub use Line::*;

// Store entry data
// File{size: N, name: ".."} -> Leaf nodes
// Dir {size: 0, name: "..."} -> Internal nodes: Parent, Children
// Parent -> Weak ownership
// Children -> Strong ownership

// Rc -> Shared ownership
// RefCell -> Interior mutability with check at runtime

type NodeRef = Rc<RefCell<Node>>;
// Children -> Strong ownership
type ChildrenRef = NodeRef;

type WeakNodeRef = Weak<RefCell<Node>>;
// Parent -> Weak ownership
type ParentRef = WeakNodeRef;

#[derive(Debug, Clone, Default)]
pub struct Node {
    size: u32,
    name: String,
    parent: Option<ParentRef>,
    children: Option<Vec<ChildrenRef>>,
}

impl Node {
    fn new_dir(name: String, parent: NodeRef) -> NodeRef {
        Node {
            size: 0,
            name,
            parent: Some(Node::weaker(parent.clone())),
            children: None,
        }
        .get_noderef()
    }

    fn root() -> NodeRef {
        Node {
            size: 0,
            name: "/".to_string(),
            parent: None,
            children: None,
        }
        .get_noderef()
    }

    fn get_weakref(self) -> WeakNodeRef {
        let nodref = Node::get_noderef(self);
        Rc::downgrade(&nodref)
    }

    fn get_noderef(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }

    fn weaker(noderef: NodeRef) -> WeakNodeRef {
        Rc::downgrade(&noderef)
    }

    fn new_file(parent: NodeRef, name: String, size: u32) -> NodeRef {
        Node {
            size,
            name,
            parent: Some(Node::weaker(parent.clone())),
            children: None,
        }
        .get_noderef()
    }

    fn update_child(whose: NodeRef, file: NodeRef) -> bool {
        // Clone the Rc to the parent node
        let parent_node = whose.clone();

        // Borrow the parent node for mutability
        if let Ok(mut parent) = parent_node.clone().try_borrow_mut() {
            if !parent.is_dir() {
                panic!("Can not add child to non-Directory")
            }

            // Create the children Vec if it doesn't exist
            if parent.children.is_none() {
                parent.children = Some(Vec::new());
            }

            // Push the file node to the children Vec
            if let Some(children) = parent.children.as_mut() {
                children.push(file.clone());
            }

            // Update the parent of the file node
            if let Ok(mut file_node) = file.try_borrow_mut() {
                file_node.parent = Some(Node::weaker(parent_node.clone()));
            }
            true
        } else {
            false
        }
    }

    fn create_dir_and_update_parent(directory: NodeRef, name: &str) -> NodeRef {
        // Create a new file node
        let dir = Node::new_dir(name.to_string(), directory.clone());

        // Update the directory to add the new dir as a child
        if Node::update_child(directory.clone(), dir.clone()) {
            directory.clone()
        } else {
            panic!("Failed to update child. The parent is not a directory.");
        }
    }

    fn create_file_and_update_parent(directory: NodeRef, name: &str, size: u32) -> NodeRef {
        // Create a new file node
        let file = Node::new_file(directory.clone(), name.to_string(), size);
        // Update the directory to add the new file as a child
        if Node::update_child(directory.clone(), file.clone()) {
            directory.clone()
        } else {
            panic!("Failed to update child. The parent is not a directory.");
        }
    }

    fn is_dir(&self) -> bool {
        self.size == 0
    }

    fn ls(noderef: NodeRef) -> Option<NodeRef> {
        if let Some(children) = noderef.clone().borrow().children.clone() {
            for child in children {
                let child = child.borrow();
                println!("{}", child);
            }
            Some(noderef.clone())
        } else {
            None
        }
    }

    fn cd(path: &str, curr: NodeRef) -> Option<NodeRef> {
        let children = curr.clone().borrow().children.clone().unwrap();
        for child in children {
            if child.borrow().name == path {
                return Some(child.clone());
            }
        }
        None
    }

    fn goto_parent(curr: NodeRef) -> Option<NodeRef> {
        if let Some(parent) = curr.clone().borrow().parent.clone() {
            parent.upgrade()
        } else {
            panic!("Unable to goto Parent from curr");
            None
        }
    }

    fn goto_root(curr: NodeRef) -> Option<NodeRef> {
        loop {
            let curr = Node::goto_parent(curr.clone());
            if curr.is_some() {
                continue;
            } else {
                break curr;
            }
        }
        // while Node::goto_parent(curr.clone()).is_some() {}
        // Some(curr)
    }
}

use std::fmt::Formatter;
impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.is_dir() {
            write!(f, " dir {}", self.name)
        } else {
            write!(f, " {} {}", self.size, self.name)
        }
    }
}

fn process_line(line: Line, curr: NodeRef) -> Option<NodeRef> {
    match line {
        Command(Cmd::Ls) => Some(curr.clone()),
        Command(Cmd::Cd(GoTo::DirName(dirname))) => Node::cd(&dirname, curr),
        Command(Cmd::Cd(GoTo::Parent)) => Node::goto_parent(curr),
        // Command(Cmd::Cd(GoTo::Root)) => Node::goto_root(curr),
        Entry(DirDescription(name)) => Some(Node::create_dir_and_update_parent(curr, &name)),
        Entry(FileDescription { name, size }) => {
            Some(Node::create_file_and_update_parent(curr, &name, size))
        }
        line => {
            dbg!(line);
            None
        }
    }
}

fn handle_terminal_data(input_str: Vec<String>) -> NodeRef {
    let mut curr = Node::root();
    let root = curr.clone();
    let mut dirs: Vec<NodeRef> = vec![root.clone()];

    for line in input_str.iter().skip(1) {
        let terminal_line = parse_line(line);
        // dbg!(terminal_line.clone());
        curr = {
            match process_line(terminal_line.clone(), curr.clone()) {
                Some(current) => {
                    if current.clone().borrow().is_dir() {
                        current
                    } else {
                        curr
                    }
                }
                None => panic!("Not able handle terminal line"),
            }
        };
    }
    root.clone()
}

#[cfg(test)]
mod test_sol1 {
    use crate::my_solution::utils::{parse_line, read_terminal};

    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day7/demo.txt";

    #[test]
    fn create_node() {
        let node = Node::root();
        dbg!(node);
    }

    #[test]
    fn test_is_dir() {
        let root = Node::root();
        let file = Node::create_file_and_update_parent(root.clone(), "ab", 999);
        assert!(root.borrow().is_dir());
        assert!(!file.borrow().is_dir());
    }

    #[test]
    fn test_ls() {
        let root = Node::root();
        Node::create_file_and_update_parent(root.clone(), "ab", 102);
        Node::create_file_and_update_parent(root.clone(), "cd", 102);
        Node::create_dir_and_update_parent(root.clone(), "x");
        Node::ls(root);
    }

    #[test]
    fn test_goto_parent() {
        let root = Node::root();
        Node::create_file_and_update_parent(root.clone(), "ab", 102);
        Node::create_file_and_update_parent(root.clone(), "cd", 102);
        let curr1 = Node::create_dir_and_update_parent(root.clone(), "x");
        let curr = Node::create_dir_and_update_parent(curr1.clone(), "y");
        // Node::create_file_and_update_parent(curr.clone(), "yd", 102);
        dbg!(Node::goto_parent(curr));
    }

    #[test]
    fn test_goto_root() {
        let root = Node::root();
        Node::create_file_and_update_parent(root.clone(), "ab", 102);
        Node::create_file_and_update_parent(root.clone(), "cd", 102);
        let curr = Node::create_dir_and_update_parent(root.clone(), "x");
        let curr = Node::create_dir_and_update_parent(curr.clone(), "y");
        Node::create_file_and_update_parent(curr.clone(), "yd", 102);
        dbg!(Node::goto_root(curr));
    }

    #[test]
    fn test_process_line() {
        let root = Node::root();
        Node::create_file_and_update_parent(root.clone(), "ab", 102);
        Node::create_file_and_update_parent(root.clone(), "cd", 102);
        let curr = Node::create_dir_and_update_parent(root.clone(), "x");
        let curr2 = Node::create_dir_and_update_parent(curr.clone(), "y");
        Node::create_file_and_update_parent(curr.clone(), "yd", 102);

        let line = Command(Ls);
        // assert_eq!(Some(&root), process_line(line, root));
        // dbg!(process_line(line, root));

        // let line = Command(Cmd::Cd(GoTo::DirName("y".to_string())));
        // let curr = process_line(line.clone(), curr.clone()).unwrap();
        // dbg!(curr.clone());

        // let line = Command(Cmd::Cd(GoTo::Parent));
        // let curr = process_line(line.clone(), curr2.clone()).unwrap();
        // dbg!(curr.clone());

        // let line = Command(Cmd::Cd(GoTo::Root));
        // let curr = process_line(line, curr2.clone()).unwrap();
        // dbg!(curr);
    }

    #[test]
    fn handle_terminal_ok() {
        let input_str = read_terminal(INPUT);
        let mut curr = Node::root();
        let root = curr.clone();
        let mut dirs: Vec<NodeRef> = vec![root.clone()];

        for line in input_str.iter().skip(1) {
            let terminal_line = parse_line(line);
            // dbg!(curr.clone().borrow().name.clone());
            // dbg!(curr.clone());
            dbg!(terminal_line.clone());
            curr = {
                match process_line(terminal_line.clone(), curr.clone()) {
                    Some(current) => {
                        if current.clone().borrow().is_dir() {
                            current
                        } else {
                            curr
                        }
                    }
                    None => panic!("Not able handle terminal line"),
                }
            };
        }
        // dbg!(root);
    }

    #[test]
    fn solution_part1() {
        let input_str = read_terminal(INPUT);
        let root = handle_terminal_data(input_str);

        
    }
}
