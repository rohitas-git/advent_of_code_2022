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

impl Hand {
    fn from_u8(val: u8) -> Option<Hand> {
        match val {
            0 => Some(Rock),
            1 => Some(Paper),
            2 => Some(Scissor),
            _ => None,
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
        }
        if opp == Paper && me == Scissor {
            return win;
        }
        if opp == Scissor && me == Rock {
            win
        } else {
            lose
        }
    }
}

use Hand::*;

pub fn total_score(file_path: &str, read_round: fn(&str) -> Result<Round, &str>) -> u32 {
    let lines = read_lines(file_path);
    let total: u32 = lines
        .iter()
        .map(|line| read_round(line).unwrap().round_score() as u32)
        .sum();
    total
}

pub fn read_round(line: &str) -> Result<Round, &str> {
    let content: Vec<&str> = line.split(" ").collect();
    let opponent = parse_move(content[0])?;
    let me = parse_move(content[1])?;
    Ok(Round { opponent, me })
}

pub fn diff_read_round(line: &str) -> Result<Round, &str> {
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
fn diff_parse_move(round: Vec<&str>) -> Result<Round, &str> {
    let h1 = round[0];
    let h2 = round[1];

    let opponent = match h1 {
        "A" | "X" => Ok(Hand::Rock),
        "B" | "Y" => Ok(Hand::Paper),
        "C" | "Z" => Ok(Hand::Scissor),
        _ => Err("Error parsing the move"),
    }
    .unwrap();

    let val = opponent.clone() as u8 - 1;
    let me = match h2 {
        "X" => Hand::from_u8(if val == 0 { 2 } else { (val - 1) % 3 }).unwrap(),
        "Y" => Hand::from_u8(val).unwrap(),
        "Z" => Hand::from_u8((val + 1) % 3).unwrap(),
        _ => return Err("Error parsing the move"),
    };

    Ok(Round { opponent, me })
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

mod another_solution {
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
    enum Move {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Outcome {
        Win,
        Draw,
        Loss,
    }

    trait MoveTrait {
        fn inherent_points(self) -> usize;
        fn beats(self, other: Move) -> bool;
        fn outcome(self, theirs: Move) -> Outcome;
    }

    trait OutcomeTrait {
        fn inherent_points(self) -> usize;
        fn matching_move(self, theirs: Move) -> Move;
    }

    impl MoveTrait for Move {
        fn inherent_points(self) -> usize {
            match self {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            }
        }

        fn beats(self, other: Move) -> bool {
            matches!(
                (self, other),
                (Self::Rock, Self::Scissors)
                    | (Self::Paper, Self::Rock)
                    | (Self::Scissors, Self::Paper)
            )
        }

        fn outcome(self, theirs: Move) -> Outcome {
            if self.beats(theirs) {
                Outcome::Win
            } else if theirs.beats(self) {
                Outcome::Loss
            } else {
                Outcome::Draw
            }
        }
    }

    impl OutcomeTrait for Outcome {
        fn inherent_points(self) -> usize {
            match self {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            }
        }

        fn matching_move(self, theirs: Move) -> Move {
            match self {
                Outcome::Win => theirs.winning_move(),
                Outcome::Draw => theirs.drawing_move(),
                Outcome::Loss => theirs.losing_move(),
            }
        }
    }

    impl Move {
        const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

        fn winning_move(self) -> Self {
            Self::ALL_MOVES
                .iter()
                .copied()
                .find(|m| m.beats(self))
                .expect("at least one move beats us")
        }

        fn losing_move(self) -> Self {
            Self::ALL_MOVES
                .iter()
                .copied()
                .find(|&m| self.beats(m))
                .expect("we beat at least one move")
        }

        fn drawing_move(self) -> Self {
            self
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Round {
        theirs: Move,
        ours: Move,
    }

    impl Round {
        fn outcome(self) -> Outcome {
            self.ours.outcome(self.theirs)
        }

        fn our_score(self) -> usize {
            self.ours.inherent_points() + self.outcome().inherent_points()
        }
    }

    mod part1 {
        use super::{Move, Outcome, Round};
        use std::str::FromStr;

        // Trait wrappers 
        struct P1Move(Move);
        struct P1Round(Round);

        // try to parse a Move from either ABC or XYZ
        impl TryFrom<char> for P1Move {
            type Error = color_eyre::Report;

            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    'A' | 'X' => Ok(P1Move(Move::Rock)),
                    'B' | 'Y' => Ok(P1Move(Move::Paper) ),
                    'C' | 'Z' => Ok(P1Move(Move::Scissors) ),
                    _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
                }
            }
        }

        // parse a Round given a &str (a single line)
        impl FromStr for P1Round {
            type Err = color_eyre::Report;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut chars = s.chars();
                let (Some(theirs), Some(' '), Some(ours), None) =
                    (chars.next(), chars.next(), chars.next(), chars.next())
                else {
                    return Err(color_eyre::eyre::eyre!(
                        "expected <theirs>SP<ours>EOF, got {s:?}"
                    ));
                };
                {
                    Ok(P1Round(Round {
                        theirs: <char as std::convert::Into<P1Move>>::into(theirs)?.0,
                        ours: ours.try_into()?,
                    }))
                }
            }
        }

        #[cfg(test)]
        mod test_main {
            use super::*;
            use itertools::{process_results, Itertools};

            #[test]
            fn main_ok() -> color_eyre::Result<()> {
                color_eyre::install()?;
                {
                    use self::*;

                    let total_score: usize = itertools::process_results(
                        include_str!("input.txt")
                            .lines()
                            .map(Round::from_str)
                            // 👇 this is provided by `Itertools`
                            .map_ok(|r| r.our_score()),
                        |it| it.sum(),
                    )?;
                    dbg!(total_score);
                }

                Ok(())
            }
        }
    }

    mod part2 {
        use super::{Move, Outcome, Round};
        use std::str::FromStr;

        impl TryFrom<char> for Move {
            type Error = color_eyre::Report;

            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    'A' => Ok(Move::Rock),
                    'B' => Ok(Move::Paper),
                    'C' => Ok(Move::Scissors),
                    _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
                }
            }
        }

        impl TryFrom<char> for Outcome {
            type Error = color_eyre::Report;

            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    'X' => Ok(Outcome::Loss),
                    'Y' => Ok(Outcome::Draw),
                    'Z' => Ok(Outcome::Win),
                    _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
                }
            }
        }

        // a helper to Outcome itself, to find the move that matches, given their move
        impl Outcome {
            fn matching_move(self, theirs: Move) -> Move {
                match self {
                    Outcome::Win => theirs.winning_move(),
                    Outcome::Draw => theirs.drawing_move(),
                    Outcome::Loss => theirs.losing_move(),
                }
            }
        }

        // change Round's FromStr implementation to parse their move, the desired outcome, and decide what our move should be
        impl FromStr for Round {
            type Err = color_eyre::Report;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut chars = s.chars();
                let (Some(theirs), Some(' '), Some(outcome), None) =
                    (chars.next(), chars.next(), chars.next(), chars.next())
                else {
                    return Err(color_eyre::eyre::eyre!(
                        "expected <theirs>SP<outcome>EOF, got {s:?}"
                    ));
                };
                {
                    use self::*;
                    let theirs = Move::try_from(theirs)?;
                    let outcome = Outcome::try_from(outcome)?;
                    let ours = outcome.matching_move(theirs);
                    Ok(Self { theirs, ours })
                }
            }
        }

        #[cfg(test)]
        mod test_main {
            use super::*;

            #[test]
            fn part2_main() {
                {
                    use self::*;
                    let total_score: usize = itertools::process_results(
                        include_str!("input.txt")
                            .lines()
                            .map(Round::from_str)
                            //    there 👇   👇
                            .map_ok(|r| dbg!(dbg!(r).our_score())),
                        |it| it.sum(),
                    )?;
                    dbg!(total_score);
                }
            }
        }
    }
}
