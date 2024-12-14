use nom::{
    bytes::complete::tag,
    multi::many0,
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<u32, String> {
    match parse(input) {
        Ok((_remaining, results)) => {
            // dbg!(&results);
            Ok(12)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("advent"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "12";
        assert_eq!(12, process(contents)?);
        Ok(())
    }
}
