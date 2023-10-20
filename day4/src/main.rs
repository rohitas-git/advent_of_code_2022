mod utils;

const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day4/src/input.txt";

fn main() {
    println!("contains: {}", utils::get_containing_pairs(INPUT).unwrap());
    println!("overlap: {}", utils::get_overlap_pairs(INPUT).unwrap());
}
