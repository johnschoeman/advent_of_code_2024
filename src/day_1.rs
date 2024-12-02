use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./day_1_input.txt";

pub fn run() -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;

    let sum = calculate_sim_score(&contents);

    Ok(sum)
}

fn calculate_sim_score(contents: &str) -> i32 {
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

    let result = left_list
        .iter()
        .map(|x| {
            let count: i32 = right_list
                .iter()
                .map(|&x| x as i32)
                .filter(|y| *y == *x)
                .collect::<Vec<i32>>()
                .len()
                .try_into()
                .unwrap();
            return x * count;
        })
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
        assert_eq!(5, calculate_sim_score(contents));
    }
}
