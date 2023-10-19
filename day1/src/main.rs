// Read the input txt file
// Parse the input and store the sum for each Elf in a Vec
// Find the maximum sum
// Find which elf is carrying the maximum sum

mod utlis;
use utlis::*;
pub use std::fs;
const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day1/src/input.txt";

fn main() {
    let mut total_calories = read_calories(INPUT);
    total_calories.sort();
    let l = total_calories.len();
    println!("1: {}, 2:{}, 3:{}", total_calories[l-1], total_calories[l-2], total_calories[l-3]);

    let mut top3 = 0;
    for i in 1..4{
        top3 += total_calories[l-i];
    }
    println!("top3: {}", top3);

}
