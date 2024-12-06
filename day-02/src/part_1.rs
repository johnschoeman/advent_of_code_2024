use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};
use std::error::Error;
use std::fs;
use tracing::{info, instrument};

const FILE_PATH: &str = "./input1.txt";

#[tracing::instrument]
pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result),
        Err(e) => Err(e.into()),
    }
}

enum Direction {
    Increasing,
    Decreasing,
}

fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed: {}", e))?;
    let result = reports
        .iter()
        .map(|report| check_safety(report))
        .filter(|safety| safety.is_ok())
        .count();
    Ok(result.to_string())
}

#[instrument]
fn check_safety(report: &Report) -> Result<(), String> {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(format!("{} {} switched to increasing", a, b));
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    direction = Some(Direction::Decreasing);
                }
            },
            1 => match direction {
                Some(Direction::Decreasing) => {
                    return Err(format!("{} {} switched to decreasing", a, b));
                }
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{} {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    direction = Some(Direction::Increasing);
                }
            },
            0 => {
                Err(format!("{} {} diff value is 0", a, b))?;
            }
            _ => {
                panic!("unexpected signum: {}", diff.signum());
            }
        };
    }
    Ok(())
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let contents = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("2", process(contents)?);
        Ok(())
    }
}
