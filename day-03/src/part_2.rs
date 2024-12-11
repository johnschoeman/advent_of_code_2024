use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, value},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

#[derive(Clone, Debug, PartialEq)]
enum Keyword {
    Mul(i32),
    Do,
    Dont,
}

#[derive(Debug, PartialEq)]
enum ShouldProcess {
    Yes,
    No,
}

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;

    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<i32, String> {
    match parse(input) {
        Ok((_remaining, instructions)) => {
            dbg!(&instructions);
            let sum = instructions
                .iter()
                .fold((0, ShouldProcess::Yes), |acc, inst| match inst {
                    Keyword::Mul(product) => {
                        if acc.1 == ShouldProcess::Yes {
                            let next_sum = acc.0 + product;
                            return (next_sum, ShouldProcess::Yes);
                        } else {
                            return acc;
                        }
                    }
                    Keyword::Do => return (acc.0, ShouldProcess::Yes),
                    Keyword::Dont => return (acc.0, ShouldProcess::No),
                });
            Ok(sum.0)
        }
        Err(_err) => {
            dbg!(_err);
            Err("parsing failed".to_string())
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Keyword>> {
    many0(map(many_till(anychar, parse_instruction), |value| {
        return value.1;
    }))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Keyword> {
    alt((
        value(Keyword::Do, tag("do()")),
        value(Keyword::Dont, tag("don't()")),
        parse_mul,
    ))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Keyword> {
    let (next, _) = tag("mul")(input)?;

    let (next, result) = delimited(
        tag("("),
        separated_pair(parse_number, tag(","), parse_number),
        tag(")"),
    )(next)?;

    let product = result.0 * result.1;
    return Ok((next, Keyword::Mul(product)));
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (next, num) = digit1(input)?;
    Ok((next, num.parse::<i32>().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(contents)?);
        Ok(())
    }
}
