use std::fs;
use std::path::Path;

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
            let calorie:u32 = item.parse::<u32>().expect("No spacing after numbers and anything similar");
            sum += calorie;
        }
    }
    calories
}

fn read_lines(file_path: &str) -> Vec<String>{
    let input_path = Path::new(file_path);
    let input = fs::read_to_string(input_path).expect("Should read input file content");

    // Split by empty line to get
    let lines: Vec<String> = input.clone().split('\n').map(|s| s.to_string()).collect();
    lines
}

#[cfg(test)]
mod test_utlis {

    use super::*;
    const INPUT: &str = "/home/user/dev/tutorial/adventOfCode/2022/day1/demo.txt";

    #[test]
    fn check_read_file() {
        let calories = read_calories(INPUT);
        assert_eq!(calories, vec![5000,4000,11000,24000]);
    }

    fn _print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
}
