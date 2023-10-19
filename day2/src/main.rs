mod utils;
use utils::*;

const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day2/src/input.txt";

fn main() {
    let mistaken_total_score = total_score(INPUT, read_round);
    println!("First Strategy Total: {}", mistaken_total_score);

    let diff_total_score = total_score(INPUT, diff_read_round);
    println!("Second Strategy Total: {}", diff_total_score);
}
