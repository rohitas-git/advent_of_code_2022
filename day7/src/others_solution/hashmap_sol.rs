// Appraoch:
// Build up a map of all directories where the key is a path,
// and the value is the size of the directory at that path.

// I kept track of 2 variables.
// One that holds the map of path to size. (the HashMap called sizes)
// One with a list of all paths that are affected if we encounter a new file. (the Vec called affected)

use std::collections::HashMap;
use std::path::PathBuf;

#[test]
pub fn part_1(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    let result: u32 = sizes.into_values().filter(|size| *size <= 100_000).sum();
    dbg!(&result);
}
