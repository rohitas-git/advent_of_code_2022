
pub enum Line {
    Command(Command),
    Entry(Entry),
}


pub enum Command {
    Cd(GoTo),
    Ls,
}

pub enum Entry {
    DirDescription(String),
    FileDescription { name: String, size: u32 },
}

pub enum GoTo {
    DirName(String),
    Parent,
    Root,
}

pub use GoTo::*;
pub use Line::*;
pub use Command::*;
pub use Entry::*;

pub fn parse_line(line: &str) -> Line {
    if line.contains("$") && line.contains("cd") {
        let content: Vec<&str> = line.split(" ").collect();
        let detail = match content[2] {
            "/" => return Command(Cd(GoTo::Root)),
            ".." => return Command(Cd(GoTo::Parent)),
            dirname => Command(Cd(GoTo::DirName(dirname.to_string()))),
        };
        return detail;
    }
    if line.contains("$") && line.contains("ls") {
        return Command(Ls);
    }
    if line.contains("dir") {
        let content: Vec<&str> = line.split(" ").collect();
        return Entry(DirDescription(content[1].to_string()));
    } else if line.is_ascii() {
        let content: Vec<&str> = line.split(" ").collect();
        return Entry(FileDescription {
            name: content[1].to_string(),
            size: content[0].parse::<u32>().expect("Parsing of file size"),
        });
    } else{
        panic!("Not able to parse line");
    }

}

pub fn read_terminal(input: &str) -> Vec<String> {
    let input_str = std::fs::read_to_string(input).expect("Able to read input file");
    let terminal_lines: Vec<String> = input_str.split("\n").map(|s| s.to_string()).collect();
    terminal_lines
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day7/demo.txt";

    #[test]
    fn test_reading_terminal() {
        let input = std::fs::read_to_string(INPUT).expect("Able to read input file");
        let terminal_lines: Vec<&str> = input.split("\n").collect();
        dbg!(terminal_lines);
    }

    #[test]
    fn test_determine_which_command() {}
}
