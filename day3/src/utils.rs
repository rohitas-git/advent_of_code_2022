#![allow(unused)]
use std::fs;
use std::path::Path;
use std::collections::HashSet;

pub fn total_priority_sum(input_path: &str) -> u32{
    let lines = read_lines(input_path);
    let mut total = 0;

    for line in lines.iter(){
        let c = shared_item(line);
        let val = priority_val(c);
        total+= val;
    }
    total 
}

pub fn get_badges_sum(file_path: &str)-> u32{
    let lines = read_lines(file_path);
    let length = lines.len();
    let mut badges: Vec<char> = Vec::new();

    let mut i =0;
    while i != length{
        let r1 = &lines[i];
        let r2 = &lines[i+1];
        let r3 = &lines[i+2];

        let set: HashSet<char> = r1.chars().collect();
        let shared: Vec<char> = r2.chars().filter(|c| set.contains(c)).collect();
        let commons: Vec<char> = r3.chars().filter(|c| shared.contains(c)).collect();

        badges.push(commons[0]);
        i+=3;
    }

    badges.into_iter().map(|c| priority_val(c)).sum()
}

fn priority_val(c: char) -> u32{
    if c.is_ascii_lowercase(){
        c as u32 - 96
    }
    else{
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

fn compartments(rucksack: &str) -> (&str,&str){
    let length = rucksack.len();
    rucksack.split_at(length/2)
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
    fn get_compartments_ok(){
        let lines = read_lines(INPUT);
        let line = lines.get(0).unwrap();
        println!("{:?}", compartments(line));
    }

    #[test]
    fn get_shared_item_ok(){
        let lines = read_lines(INPUT);
        let line = lines.get(0).unwrap();
        let shared = shared_item(line);
        println!("{:?}", shared as u32);
    }

    #[test]
    fn get_val_ok(){
        assert_eq!(1, priority_val('a'));
        assert_eq!(26, priority_val('z'));
        assert_eq!(52, priority_val('Z'));
        assert_eq!(27, priority_val('A'));
    }

    #[test]
    fn get_badges_sum_ok(){
        assert_eq!(70, get_badges_sum(INPUT));
    }

    #[test]
    fn get_total_ok(){
        assert_eq!(157,total_priority_sum(INPUT));
    }
}
