#![allow(unused)]

use std::char::from_u32;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock = 1,
    Paper,
    Scissor,
}

impl Hand{
    fn from_u8(val: u8) -> Option<Hand> {
        match val {
            0 => return Some(Rock),
            1 => return Some(Paper),
            2 => return Some(Scissor),
            _ => return None
        }
    }
}

#[derive(Debug)]
pub struct Round {
    opponent: Hand,
    me: Hand,
}

impl Round {
    fn round_score(&self) -> u8 {
        let round = self.to_owned();
        let opp = round.opponent.clone();
        let me = round.me.clone();

        let score = me.clone() as u8;
        let tie = score + 3;
        let win = score + 6;
        let lose = score;

        // Tie
        if opp == me {
            return tie;
        }

        // If Win else Lose
        if opp == Rock && me == Paper {
            return win;
        } else if opp == Paper && me == Scissor {
            return win;
        } else if opp == Scissor && me == Rock {
            return win;
        } else {
            return lose;
        }
    }
}

use Hand::*;

pub fn total_score(file_path: &str, read_round: fn(&String)-> Result<Round, &str>) -> u32 {
    let lines = read_lines(file_path);
    let total: u32 = lines
        .iter()
        .map(|line| read_round(line).unwrap().round_score() as u32)
        .sum();
    total
}

pub fn read_round(line: &String) -> Result<Round, &str> {
    let content: Vec<&str> = line.split(" ").collect();
    let opponent = parse_move(content[0])?;
    let me = parse_move(content[1])?;
    Ok(Round { opponent, me })
}

pub fn diff_read_round(line: &String) -> Result<Round, &str> {
    let content: Vec<&str> = line.split(" ").collect();
    let round = diff_parse_move(content)?;
    Ok(round)
}


fn parse_move(choice: &str) -> Result<Hand, &str> {
    match choice {
        "A" | "X" => Ok(Hand::Rock),
        "B" | "Y" => Ok(Hand::Paper),
        "C" | "Z" => Ok(Hand::Scissor),
        _ => Err("Error parsing the move"),
    }
}

// parse move according to the different strategy
fn diff_parse_move(round: Vec<&str>) -> Result<Round, &str>{
    let h1 = round[0];
    let h2 = round[1];

    let opponent = match h1 {
        "A" | "X" => Ok(Hand::Rock),
        "B" | "Y" => Ok(Hand::Paper),
        "C" | "Z" => Ok(Hand::Scissor),
        _ => Err("Error parsing the move"),
    }.unwrap();

    let val = opponent.clone() as u8 - 1;
    let me = match h2 {
        "X" => Hand::from_u8(if val==0{2} else {(val-1)%3}).unwrap()  ,
        "Y" => Hand::from_u8(val).unwrap(),
        "Z" => Hand::from_u8((val+1)%3).unwrap(),
        _ => return Err("Error parsing the move"),
    };

    Ok(Round{opponent, me})
}

fn read_lines(file_path: &str) -> Vec<String> {
    let input_path = Path::new(file_path);
    let input = fs::read_to_string(input_path).expect("Should read input file content");

    // Split by empty line to get
    let lines: Vec<String> = input.clone().split('\n').map(|s| s.to_string()).collect();
    lines
}

#[cfg(test)]
mod test_utils {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day2/demo.txt";

    #[test]
    fn ok_read_lines() {
        println!("{:?}", _print_type_of(&read_lines(INPUT)[1]));
    }

    fn _print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn ok_parse_round() {
        let hand = parse_move("A").unwrap();
        assert_eq!(Rock, hand);

        let hand = parse_move("B").unwrap();
        assert_eq!(Paper, hand);

        let hand = parse_move("C").unwrap();
        assert_eq!(Scissor, hand);
    }

    #[test]
    fn ok_read_round() {
        let first = read_lines(INPUT).get(0).unwrap().clone();
        let round = read_round(&first).unwrap();
        println!("{:?}", round)
    }

    #[test]
    fn ok_round_score() {
        let round = Round {
            opponent: Rock,
            me: Paper,
        };
        assert_eq!(8, round.round_score());

        let round = Round {
            opponent: Scissor,
            me: Paper,
        };
        assert_eq!(2, round.round_score());

        let round = Round {
            opponent: Paper,
            me: Paper,
        };
        assert_eq!(5, round.round_score());
    }

    #[test]
    fn ok_totals() {
        let total = total_score(INPUT, read_round);
        assert_eq!(15, total);

        let total = total_score(INPUT, diff_read_round);
        assert_eq!(12, total);
    
    }


}
