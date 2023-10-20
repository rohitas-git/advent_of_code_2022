use std::fs;
use std::path::Path;

pub mod another_solution {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1#the-problem-statement

    use itertools::{FoldWhile, Itertools};
    use std::cmp::Reverse;
    pub fn top_3() -> color_eyre::Result<()> {
        color_eyre::install()?;

        let answer = include_str!("input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .batching(|it| it.map_while(|x| x).sum1::<u64>())
            .map(Reverse)
            .k_smallest(3)
            .map(|x| x.0)
            .sum::<u64>();
        println!("{answer:?}");

        Ok(())
    }

    pub fn total_calories() -> u64 {
        let lines = include_str!("input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok());

        let elven_lead = GroupSumIter { inner: lines }.max();
        elven_lead.unwrap()
    }

    struct GroupSumIter<I> {
        inner: I,
    }

    impl<I> Iterator for GroupSumIter<I>
    where
        I: Iterator<Item = Option<u64>>,
    {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let mut sum = loop {
                match self.inner.next() {
                    Some(Some(v)) => break v,
                    Some(None) => {
                        // huh, weird, didn't expect a separator there
                        // but let's just skip it
                    }
                    // we've reached the end of the inner iterator
                    None => return None,
                }
            };

            loop {
                match self.inner.next() {
                    Some(Some(v)) => sum += v,
                    Some(None) | None => {
                        // reached a separator or the end of the iterator
                        break Some(sum);
                    }
                }
            }
        }
    }
}

pub fn read_calories(input: &str) -> Vec<u32> {
    let mut cals_str = read_lines(input);
    cals_str.push("".to_string());

    let mut calories: Vec<u32> = Vec::new();
    let mut sum = 0;

    // parse each calorie and sum those in a grp and push them to a vec
    for item in cals_str.into_iter() {
        if item == "" {
            calories.push(sum);
            sum = 0;
        } else {
            let calorie: u32 = item
                .parse::<u32>()
                .expect("No spacing after numbers and anything similar");
            sum += calorie;
        }
    }
    calories
}

fn read_lines(file_path: &str) -> Vec<String> {
    let input_path = Path::new(file_path);
    let input = fs::read_to_string(input_path).expect("Should read input file content");

    // Split by empty line to get
    let lines: Vec<String> = input.clone().split("\n").map(|s| s.to_string()).collect();
    lines
}

#[cfg(test)]
mod test_utlis {

    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day1/demo.txt";

    #[test]
    fn check_total_calories() {
        let total = another_solution::total_calories();
        println!("{:?}", total);
    }

    #[test]
    fn check_read_lines() {
        let lines: Vec<&str> = include_str!("../demo.txt").split("\n").collect();
        println!("{:?}", lines);
    }

    #[test]
    fn check_read_file() {
        let calories = read_calories(INPUT);
        assert_eq!(calories, vec![5000, 4000, 11000, 24000]);
    }

    fn _print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
}
