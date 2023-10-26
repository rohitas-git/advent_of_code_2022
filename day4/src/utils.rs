#![allow(unused)]

use std::{any::Any, fs};

type AnyResult<T> = Result<T, anyhow::Error>;

#[derive(Debug)]
struct Area {
    start: u32,
    end: u32,
}

impl Area {
    fn contains(&self, other: &Self) -> bool {
        let l_start = self.start < other.start;
        let g_end = self.end > other.end;

        if other.start == self.start || other.end == self.end {
            return true;
        }

        if (g_end && l_start) || (!g_end && !l_start) {
            return true;
        }
        false
    }

    fn overlap(&self, other: &Self) -> bool {
        let p1 = self.start;
        let p2 = self.end;

        if p1 >= other.start && p1 <= other.end {
            return true;
        }
        if p2 >= other.start && p2 <= other.end {
            return true;
        }
        self.contains(other)
    }
}

pub fn get_overlap_pairs(input: &str) -> AnyResult<u32> {
    let areas = get_areas(input)?;
    let count: u32 = areas
        .into_iter()
        .map(|pair| if (pair.0).overlap(&(pair.1)) { 1 } else { 0 })
        .sum();
    Ok(count)
}

pub fn get_containing_pairs(input: &str) -> AnyResult<u32> {
    let areas = get_areas(input)?;
    let count: u32 = areas
        .into_iter()
        .map(|pair| if (pair.0).contains(&(pair.1)) { 1 } else { 0 })
        .sum();
    Ok(count)
}

fn get_areas(input: &str) -> AnyResult<Vec<(Area, Area)>> {
    
    // Gives String [done at runtime => arg: string variable]
    let input = fs::read_to_string(input)?;

    // Gives &'static str [done at compile time => req arg: String literal]
    // let input = include_str!("input.txt");

    let mut areas: Vec<(Area, Area)> = Vec::new();

    for line in input.lines() {
        let areas_str: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = areas_str[0].split('-').collect();
        let second: Vec<&str> = areas_str[1].split('-').collect();

        let first_area = {
            let start = first[0].parse::<u32>()?;
            let end = first[1].parse::<u32>()?;
            Area { start, end }
        };

        let second_area = {
            let start = second[0].parse::<u32>()?;
            let end = second[1].parse::<u32>()?;
            Area { start, end }
        };

        let pair = (first_area, second_area);
        areas.push(pair);
    }

    Ok(areas)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day4/demo.txt";

    #[test]
    fn test_get_areas() {
        let areas = get_areas(INPUT).unwrap();
        let area = &areas[2];
        println!("{:?}", area);
    }

    #[test]
    fn test_contains() {
        let areas = get_areas(INPUT).unwrap();
        let area = &areas[4];
        let first = &area.0;
        let second = &area.1;
        println!("{:?}", first.contains(second));
    }

    #[test]
    fn get_count_containing_pairs_ok() {
        let c = get_containing_pairs(INPUT);
        assert_eq!(2, c.unwrap());
    }

    #[test]
    fn get_overlap_pairs_ok() {
        let c = get_overlap_pairs(INPUT);
        assert_eq!(4, c.unwrap());
    }
}
