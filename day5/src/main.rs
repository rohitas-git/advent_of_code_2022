pub mod item;
pub mod stack;

use item::*;
use stack::*;

use day5::read_input;

const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/input.txt";

fn main() -> Result<(), color_eyre::Report> {
    let (setup_str, moves_str) = read_input(INPUT).unwrap();
    let mut stacks = Stacks::from_arrangement(setup_str.clone())?;
    let mut stacks_2 = Stacks::from_arrangement(setup_str.clone())?;

    let all_moves = Move::read_moves(moves_str);

    // PART 1
    for todo in all_moves.iter() {
        stacks.do_move_single_crate(todo);
    }
    stacks.get_stack_tops();

    // PART 2
    for todo in all_moves.iter() {
        stacks_2.do_move_multiple_crates(todo);
    }
    stacks_2.get_stack_tops();
    
    Ok(())
}
