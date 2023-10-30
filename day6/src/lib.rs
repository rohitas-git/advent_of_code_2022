#![allow(unused)]

use std::{error::Error, fs};
pub const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day6/src/demo.txt";

/// true if no no_repeated_chars in input
fn no_repeated_chars(input: &str) -> bool {
    let mut chars: Vec<u8> = input.as_bytes().to_vec();

    while !chars.is_empty() {
        while let Some(popped) = chars.pop() {
            if chars.contains(&popped) {
                return false;
            }
        }
    }
    true
}


pub fn solution_v1(){
    let result: Vec<usize> = include_str!("input.txt").lines().map(|line| {
        let jump: usize = 14; // flip to 4 for part 1, 14 for part2
        let mut index: usize = 0;
        let mut result = 0;

        let input_chars = line.chars().collect::<Vec<char>>();
        while index + jump < input_chars.len() {
            let chunk: String = input_chars[index..(index + jump)].iter().collect();

            if no_repeated_chars(&chunk) {
                result = index + jump;
                break;
            }
            index += 1;
        }
        // dbg!(&line, &result, &index);
        result
    }).collect();

    dbg!(result);
}



#[cfg(test)]
mod test_sigals {
    use super::*;

    #[test]
    fn test_signal_lock() {
        solution_v1();
    }
}
