#![allow(unused)]

mod item;
mod stack;

use item::*;
use stack::*;

pub fn read_input(file_path: &str) -> Result<(Vec<String>, Vec<String>), color_eyre::Report> {
    use std::fs;
    if let Ok(input) = fs::read_to_string(file_path) {
        let input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

        let arrangement: Vec<String> = input[0]
            .clone()
            .split('\n')
            .map(|s| s.to_string())
            .collect();

        let moves: Vec<String> = input[1]
            .clone()
            .split('\n')
            .map(|s| s.to_string())
            .collect();

        Ok((arrangement, moves))
    } else {
        Err(color_eyre::eyre::eyre!("Error reading the input file"))
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/input.txt";

    #[test]
    fn read_items_ok() {
        let (setup_str, moves_str) = read_input(INPUT).unwrap();
        dbg!(moves_str);
    }

    
}
