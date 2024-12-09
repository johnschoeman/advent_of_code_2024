use nom::{
    bytes::complete::{is_a, is_not, tag, take},
    character::complete::digit1,
    multi::many0,
    sequence::preceded,
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
            // println!("remaining: {:?}", _remaining);
            let sum = results.iter().sum::<i32>();
            Ok(sum)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

type Output = i32;

fn parse(input: &str) -> IResult<&str, Vec<Output>> {
    many0(parse_mul)(input)
}

fn parse_number(input: &str) -> IResult<&str, Output> {
    let (next, num) = digit1(input)?;
    Ok((next, num.parse::<i32>().unwrap()))
}

type NomErr<'a> = nom::error::Error<&'a str>;

fn parse_mul(input: &str) -> IResult<&str, Output> {
    let (next, _) = match preceded(many0(is_not::<&str, &str, NomErr>("m")), tag("mul"))(input) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match mul");
            let (next, _) = take(1usize)(input)?;
            return Ok((next, 0));
        }
    };

    let (next, _) = match is_a::<&str, &str, NomErr>("(")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match (");
            return Ok((next, 0));
        }
    };

    let (next, first_num) = parse_number(next)?;

    let (next, _) = match is_a::<&str, &str, NomErr>(",")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match ,");
            return Ok((next, 0));
        }
    };

    let (next, second_num) = parse_number(next)?;

    let (next, _) = match is_a::<&str, &str, NomErr>(")")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match )");
            return Ok((next, 0));
        }
    };

    let result = first_num * second_num;
    Ok((next, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(contents)?);
        Ok(())
    }
}
