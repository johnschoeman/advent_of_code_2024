use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take},
    character::complete::{digit1, none_of},
    combinator::map,
    multi::{fold_many0, many0},
    sequence::preceded,
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

#[derive(Debug, PartialEq)]
enum Keyword {
    Do,
    Dont,
    Mul,
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
        Ok((_remaining, results)) => {
            // dbg!(&results);
            let sum = results
                .iter()
                .fold((0, Keyword::Do), |acc, x| match x.1 {
                    Keyword::Mul => {
                        if acc.1 == Keyword::Do {
                            let next_sum = acc.0 + x.0;
                            return (next_sum, Keyword::Do);
                        } else {
                            return acc;
                        }
                    }
                    Keyword::Do => return (acc.0, Keyword::Do),
                    Keyword::Dont => return (acc.0, Keyword::Dont),
                });
            Ok(sum.0)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}


type Value = i32;
type Output = (Value, Keyword);

fn parse(input: &str) -> IResult<&str, Vec<Output>> {
    fold_many0(parse_mul, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        return acc;
    })(input)
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    let (next, num) = digit1(input)?;
    Ok((next, num.parse::<Value>().unwrap()))
}

type NomErr<'a> = nom::error::Error<&'a str>;

fn parse_mul(input: &str) -> IResult<&str, Output> {
    let (next, keyword_match) = match preceded(
        many0(none_of::<&str, &str, NomErr>("md")),
        alt((
            map(tag("mul"), |_s: &str| Keyword::Mul),
            map(tag("do()"), |_s: &str| Keyword::Do),
            map(tag("don't()"), |_s: &str| Keyword::Dont),
        )),
    )(input)
    {
        Ok((next, matched)) => (next, (0, matched)),
        Err(_) => {
            // println!("Didn't match mul");
            let (next, _) = take(1usize)(input)?;
            let output = (0, Keyword::Mul);
            return Ok((next, output));
        }
    };

    if keyword_match.1 == Keyword::Do {
        return Ok((next, (0, Keyword::Do)));
    }
    if keyword_match.1 == Keyword::Dont {
        return Ok((next, (0, Keyword::Dont)));
    }

    let (next, _) = match is_a::<&str, &str, NomErr>("(")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match (");
            let output = (0, Keyword::Mul);
            return Ok((next, output));
        }
    };

    let (next, first_num) = parse_number(next)?;

    let (next, _) = match is_a::<&str, &str, NomErr>(",")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match ,");
            let output = (0, Keyword::Mul);
            return Ok((next, output));
        }
    };

    let (next, second_num) = parse_number(next)?;

    let (next, _) = match is_a::<&str, &str, NomErr>(")")(next) {
        Ok((next, _)) => (next, 0),
        Err(_) => {
            // println!("Didn't match )");
            let output = (0, Keyword::Mul);
            return Ok((next, output));
        }
    };

    let result = first_num * second_num;
    let output = (result, Keyword::Mul);
    Ok((next, output))
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
