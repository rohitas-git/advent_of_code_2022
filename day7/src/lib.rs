mod directory;

enum Line {
    Cd(CdOption),
    Ls,
    DirDescription(String),
    FileDescription { name: String, size: u32 },
}

enum CdOption {
    GoToDirName(String),
    GoToParent,
    GoToRoot,
}

use Line::*;

fn is_what(line: &str) -> Line {
    if line.contains("$") && line.contains("cd") {
        let content: Vec<&str> = line.split(" ").collect();
        let detail = match content[2] {
            "/" => return Cd(CdOption::GoToRoot),
            ".." => return Cd(CdOption::GoToParent),
            dirname => Cd(CdOption::GoToDirName(dirname.to_string())),
        };
        return detail;
    }
    if line.contains("$") && line.contains("ls") {
        return Ls;
    }
    if line.contains("dir") {
        let content: Vec<&str> = line.split(" ").collect();
        return DirDescription(content[0].to_string());
    } else {
        let content: Vec<&str> = line.split(" ").collect();
        return FileDescription {
            name: content[1].to_string(),
            size: content[0].parse::<u32>().expect("Parsing of file size"),
        };
    }
}

fn read_terminal(input: &str) -> Vec<String> {
    let input_str = std::fs::read_to_string(input).expect("Able to read input file");
    let terminal_lines: Vec<String> = input_str.clone().split("\n").map(|s| s.to_string()).collect();
    terminal_lines.clone()
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
    fn test_determine_which_command() {
        
    }
}
