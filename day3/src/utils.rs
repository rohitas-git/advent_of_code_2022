#![allow(unused)]
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn total_priority_sum(input_path: &str) -> u32 {
    let lines = read_lines(input_path);
    let mut total = 0;

    for line in lines.iter() {
        let c = shared_item(line);
        let val = priority_val(c);
        total += val;
    }
    total
}

pub fn get_badges_sum(file_path: &str) -> u32 {
    let lines = read_lines(file_path);
    let length = lines.len();
    let mut badges: Vec<char> = Vec::new();

    let mut i = 0;
    while i != length {
        let r1 = &lines[i];
        let r2 = &lines[i + 1];
        let r3 = &lines[i + 2];

        let set: HashSet<char> = r1.chars().collect();
        let shared: Vec<char> = r2.chars().filter(|c| set.contains(c)).collect();
        let commons: Vec<char> = r3.chars().filter(|c| shared.contains(c)).collect();

        badges.push(commons[0]);
        i += 3;
    }

    badges.into_iter().map(|c| priority_val(c)).sum()
}

fn priority_val(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn shared_item(rucksack: &str) -> char {
    let compartments = compartments(rucksack);
    let n = compartments.0.len();

    let mut set: HashSet<char> = compartments.0.chars().collect();
    let mut shared: Vec<char> = compartments.1.chars().filter(|c| set.contains(c)).collect();

    shared[0]
}

fn compartments(rucksack: &str) -> (&str, &str) {
    let length = rucksack.len();
    rucksack.split_at(length / 2)
}

fn read_lines(file_path: &str) -> Vec<String> {
    let input_path = Path::new(file_path);
    let input = fs::read_to_string(input_path).expect("Should read input file content");

    // Split by empty line to get
    let lines: Vec<String> = input.clone().split('\n').map(|s| s.to_string()).collect();
    lines
}

mod test_utils {
    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day3/demo.txt";

    #[test]
    fn ok_read_lines() {
        println!("{:?}", read_lines(INPUT));
    }

    #[test]
    fn get_compartments_ok() {
        let lines = read_lines(INPUT);
        let line = lines.get(0).unwrap();
        println!("{:?}", compartments(line));
    }

    #[test]
    fn get_shared_item_ok() {
        let lines = read_lines(INPUT);
        let line = lines.get(0).unwrap();
        let shared = shared_item(line);
        println!("{:?}", shared as u32);
    }

    #[test]
    fn get_val_ok() {
        assert_eq!(1, priority_val('a'));
        assert_eq!(26, priority_val('z'));
        assert_eq!(52, priority_val('Z'));
        assert_eq!(27, priority_val('A'));
    }

    #[test]
    fn get_badges_sum_ok() {
        assert_eq!(70, get_badges_sum(INPUT));
    }

    #[test]
    fn get_total_ok() {
        assert_eq!(157, total_priority_sum(INPUT));
    }
}

mod another_solution {

    mod item {
        // to represent item in rucksack i.e char => u8 with added gurantee to be char
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub(crate) struct Item(u8);

        // convert from u8 to Item fallibly
        impl TryFrom<u8> for Item {
            type Error = color_eyre::Report;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                    _ => Err(color_eyre::eyre::eyre!(
                        "{} is not a valid item",
                        value as char
                    )),
                }
            }
        }

        impl std::fmt::Debug for Item {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0 as char)
            }
        }

        impl Item {
            /// get an item's "priority",
            pub(crate) fn score(self) -> usize {
                match self {
                    Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                    Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                    _ => unreachable!(),
                }
            }
        }
    }

    #[cfg(test)]
    mod test_main {
        use super::*;
        use item::Item;
        use itertools::Itertools;

        #[test]
        fn check_part1() -> color_eyre::Result<()> {
            let mut total_score = 0;

            for line in include_str!("input.txt").lines() {
                let (first, second) = line.split_at(line.len() / 2);

                let first_items = first
                    .bytes()
                    .map(Item::try_from)
                    .collect::<Result<Vec<_>, _>>()?;

                let dupe_score = second
                    .bytes()
                    .map(Item::try_from)
                    .find_map(|item| {
                        item.ok().and_then(|item| {
                            first_items
                                .iter()
                                // the iterator gives us `&Item`, but we want `Item`, and
                                // it's `Copy`, so we can call `copied()`
                                .copied()
                                // `find` gives us an `&Item`, but we want `Item`, so we
                                // destructure the reference here:
                                //    👇
                                .find(|&first_item| first_item == item)
                        })
                    })
                    .expect("there should be exactly one duplicate")
                    .score();
                dbg!(dupe_score);
                total_score += dupe_score;
            }

            dbg!(total_score);
            Ok(())
        }

        #[test]
        fn check_part2() -> color_eyre::Result<()> {
            // use im::HashSet;
            use itertools::Itertools;
            use std::collections::HashSet;

            let rucksacks = include_str!("input.txt").lines().map(|line| {
                line.bytes()
                    .map(Item::try_from)
                    .collect::<Result<HashSet<_>, _>>()
            });

            let sum = itertools::process_results(rucksacks, |rs| {
                rs.tuples()
                    .map(|(a, b, c)| {
                        a.iter()
                            .copied()
                            .find(|i| b.contains(i) && c.contains(i))
                            .map(|i| dbg!(i.score()))
                            .unwrap_or_default()
                    })
                    .sum::<usize>()
            })?;
            dbg!(sum);

            Ok(())
        }

        #[test]
        fn check_part2_chunks() -> color_eyre::Result<()> {
            use im::HashSet;

            let sum: usize = include_str!("input.txt")
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|b| b.try_into().unwrap())
                        .collect::<HashSet<Item>>()
                })
                .chunks(3)
                .into_iter()
                .map(|chunks| {
                    chunks
                        .reduce(|a, b| a.intersection(b))
                        .expect("we always have 3 chunks")
                        .iter()
                        .next()
                        .expect("problem statement says there is always one item in common")
                        .score()
                })
                .sum();
            dbg!(sum);

            Ok(())
        }

        #[test]
        fn check_part2_u8_array() {
            use itertools::Itertools;
            let sum: usize = include_str!("input.txt")
                .lines()
                .map(|line| {
                    line.bytes().map(|b| b.try_into().unwrap()).fold(
                        [0u8; 53],
                        |mut acc, x: Item| {
                            // this might panic!
                            acc[x.score()] = 1;
                            acc
                        },
                    )
                })
                .chunks(3)
                .into_iter()
                .map(|chunks| {
                    chunks
                        .reduce(|mut a, b| {
                            // another trick: we're re-using `a` as the output array
                            for (a, b) in a.iter_mut().zip(b.iter()) {
                                *a += *b;
                            }
                            a
                        })
                        .expect("we always have 3 chunks")
                        .iter()
                        .position(|&b| b == 3)
                        .expect("problem statement says there is always one item in common")
                })
                .sum();
            dbg!(sum);
        }
    }
}
