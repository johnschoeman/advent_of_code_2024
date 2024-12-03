use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

pub fn run() -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;

    let sum = count_safe_reports(&contents);

    Ok(sum)
}

fn count_safe_reports(contents: &str) -> i32 {
    let result = contents.lines().filter(|line| is_safe_report(line)).count() as i32;
    result
}

fn is_safe_report(line: &str) -> bool {
    let words: Vec<&str> = line.split_whitespace().collect();
    let numbers = words
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let all_desc_or_asc = is_desc_or_asc(&numbers);
    let all_within_bounds = is_all_within_bounds(&numbers);

    if all_desc_or_asc && all_within_bounds {
        return true;
    }

    for index in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(index);
        if is_desc_or_asc(&new_numbers) && is_all_within_bounds(&new_numbers) {
            return true;
        } else {
            continue;
        }
    }

    return false;
}

fn is_desc_or_asc(numbers: &Vec<i32>) -> bool {
    let mut is_desc = true;
    let mut is_asc = true;
    for i in 0..numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            is_desc = false;
        }
        if numbers[i] > numbers[i + 1] {
            is_asc = false;
        }
    }
    is_desc || is_asc
}

fn is_all_within_bounds(numbers: &Vec<i32>) -> bool {
    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i] - numbers[i + 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_safe_reports_test() {
        let contents = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(4, count_safe_reports(contents));
    }
}
