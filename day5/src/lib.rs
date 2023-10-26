struct Stack(Vec<char>);

struct Stacks {
    stacks: Vec<Stack>,
    number: u32,
}

impl Stacks {
    fn new() -> Stacks {
        Stacks {
            stacks: Vec::new(),
            number: 0,
        }
    }

    fn add_stack(&mut self, stack: Stack) {
        self.stacks.push(stack);
        self.number += 1;
    }

    fn get_initial_arrangement() -> Stacks {
        let mut stacks = Stacks::new();
        for line in include_str!("demo.txt").lines() {}

        stacks
    }
}

trait StackTrait {
    fn move_stack(arrangement: &mut Stacks, amount: u32, from_stack: usize, to_stack: usize);
}

#[derive(Debug, Clone)]
enum MyError {
    IoError(String),
}
use MyError::*;

fn read_lines(file_path: &str) -> Result<(Vec<String>, Vec<String>), MyError> {
    use std::fs;
    if let Ok(input) = fs::read_to_string(file_path) {
        let (arrange, moves): (String, String) = input.split("\n\n").map(|s| s.to_string()).collect();
        let lines: Vec<String> = input.clone().split('\n').map(|s| s.to_string()).collect();
        Ok(lines)
    } else {
        Err(IoError("Unable to read input file".into()))
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/demo.txt";

    #[test]
    fn test_lines() {
        let lines = read_lines(INPUT).unwrap();
        println!("{:?}", lines[0]);
        let (arrangement, moves) = lines.split("\n\n");
       
    }
}
