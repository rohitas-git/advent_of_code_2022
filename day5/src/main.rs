pub mod item;
pub mod stack;

use item::*;
use stack::*;

use day5::read_input;

const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/input.txt";

fn main() -> Result<(), color_eyre::Report> {
    let (setup_str, moves_str) = read_input(INPUT).unwrap();
    let mut stacks = Stacks::from_arrangement(setup_str)?;
    let all_moves = Move::read_moves(moves_str);

    for todo in all_moves {
        stacks.do_move(todo);
    }

    dbg!(stac)

    Ok(())
}
