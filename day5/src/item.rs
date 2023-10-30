
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
        }
    }
}

impl Item {
    pub fn get_char(&self) -> char {
        self.0 as char 
    }

    pub fn get_crate(item_str: &str) -> Result<Item, color_eyre::Report> {
        let item_str = item_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let item = item_str[0];

        if item.chars().count() != 3 {
            return Err(color_eyre::eyre::eyre!("Item should have format: '[A]'"));
        }

        let mut chars = item.chars();
        let _ch = chars.next();
        let ch = chars.next().expect("Able to get char from item string") as u8;

        let item = Item::try_from(ch)?;
        Ok(item)
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}


#[derive(Debug, Clone)]
pub struct Move{
    pub number: u32,
    pub from_stack: usize,
    pub to_stack: usize
}

impl Move{
    fn create(number: u32, from_stack: usize, to_stack: usize) -> Move{
        Move { number, from_stack, to_stack }
    }

    pub fn read_moves(moves_str: Vec<String>) -> Vec<Move>{
        let mut all_moves = Vec::new();

        for move_str in moves_str.into_iter() {
            let this_move: Vec<u32> = move_str.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect();
            let number = this_move[0];
            let from_stack = this_move[1] as usize;
            let to_stack = this_move[2] as usize;

            let this_move = Move::create(number, from_stack, to_stack);

            all_moves.push(this_move);
        }

        // dbg!(all_moves.clone());
        all_moves
    }
}


#[cfg(test)]
mod test_item {
    use super::*;
    use crate::read_input;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day5/src/input.txt";

    #[test]
    fn test_reading_moves() {
        let (setup_str, moves_str) = read_input(INPUT).unwrap();
        dbg!(Move::read_moves(moves_str));
    }
}