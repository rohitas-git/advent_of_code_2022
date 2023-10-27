#![allow(unused)]

use itertools::Itertools;

use crate::item::*;

#[derive(Debug, Clone)]
pub struct Stack(Vec<Item>);

impl Stack {
    fn pop(&mut self) -> Option<Item> {
        self.0.pop()
    }

    fn push_str(&mut self, item_str: &str) -> Result<(), color_eyre::Report> {
        let item = Item::get_crate(item_str)?;
        self.0.push(item);
        Ok(())
    }

    fn push(&mut self, item: Item) -> Result<(), color_eyre::Report> {
        self.0.push(item);
        Ok(())
    }

    fn new() -> Stack {
        Stack(Vec::new())
    }
}

#[derive(Debug, Clone)]
pub struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    pub fn new() -> Stacks {
        Stacks { stacks: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn add_stack(&mut self, stack: Stack) {
        self.stacks.push(stack);
    }

    pub fn from_arrangement(setup_str: Vec<String>) -> Result<Stacks, color_eyre::Report> {
        let mut stacks = Stacks::new();
        let num_crates = setup_str.len();

        stacks.stacks = vec![Stack::new(); num_crates];

        for line in setup_str.into_iter().rev().skip(1) {
            let item_opt: Vec<Option<Item>> = line
                .chars()
                .tuples()
                .map(|(a, b, c, d)| match (a, b, c, d) {
                    ('[', word @ 'A'..='Z', ']', _) => Some(Item::try_from(word as u8).unwrap()),
                    (' ', ' ', ' ', ' ') => None,
                    _ => None,
                })
                .collect();

            for (pos, item) in item_opt.into_iter().enumerate() {
                if let Some(item) = item {
                    stacks.stacks[pos].push(item);
                }
            }
        }
        Ok(stacks)
    }

    pub fn do_move(&mut self, todo: Move){
        
        for i in 1..todo.number{
            let popped = self.stacks[todo.from_stack].pop().expect("Enough items to pop");
            self.stacks[todo.to_stack].push(popped);
        }
    }

    pub fn stack_tops(&self) -> Vec<Item>{
        let mut tops = Vec::new();

        for stack in self.stacks.iter(){
            // tops.push(stack.last())
        }

        tops
    }
}

trait StackTrait {
    fn move_crates(arrangement: &mut Stacks, amount: u32, from_stack: usize, to_stack: usize);
}

#[cfg(test)]
mod test_stacks {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/input.txt";
    use crate::read_input;

    #[test]
    fn get_setup_from_input() {
        let (setup_str, moves_str) = read_input(INPUT).unwrap();
        dbg!(setup_str);
    }

    #[test]
    fn get_moves_from_input() {
        let (setup_str, moves_str) = read_input(INPUT).unwrap();
        dbg!(moves_str);
    }

    #[test]
    fn get_stack_arrangement() {
        let (setup_str, moves_str) = read_input(INPUT).unwrap();
        dbg!(Stacks::from_arrangement(setup_str));
    }
}
