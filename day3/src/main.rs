mod utils;
use utils::{total_priority_sum, get_badges_sum};
const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day3/src/input.txt";


fn main() {
    println!("Total: {}", total_priority_sum(INPUT));
    println!("Badges sum: {}", get_badges_sum(INPUT));
}
