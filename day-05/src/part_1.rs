use nom::{
    bytes::complete::tag,
    multi::many0,
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<i32, String> {
    match parse(input) {
        Ok((_remaining, results)) => {
            // dbg!(&results);
            Ok(12)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (_next, parts) = separated_list1(newline)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(12, process(contents)?);
        Ok(())
    }
}




