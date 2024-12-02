use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./day_1_input.txt";

pub fn run() -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;

    let sum = calculate_sum(&contents);

    Ok(sum)
}

fn calculate_sum(contents: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let numbers = words
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    left_list.sort();
    right_list.sort();
    let result = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let contents = "\
1 2
5 4
3 1
1 3
";
        assert_eq!(2, calculate_sum(contents));
    }
}
