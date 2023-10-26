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

    fn get_initial_arrangement(input: Vec<String>) -> Stacks {
        let mut stacks = Stacks::new();
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

fn read_input(file_path: &str) -> Result<(Vec<String>, Vec<String>), MyError> {
    use std::fs;
    if let Ok(input) = fs::read_to_string(file_path) {
        let input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
        let arrangement: Vec<String> = input[0].clone().split('\n').map(|s| s.to_string()).collect();
        let moves: Vec<String> = input[0].clone().split('\n').map(|s| s.to_string()).collect();
        Ok((arrangement, moves))
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
       let (setup, moves) = read_input(INPUT).unwrap();
       println!("{:?}", setup[setup.len() - 1]);
    }
}
