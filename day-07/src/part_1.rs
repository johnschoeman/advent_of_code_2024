use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

type Equation = (u64, Vec<u64>);

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<u64, String> {
    match parse(input) {
        Ok((_remaining, results)) => {
            // dbg!(&results);
            let result = results.iter().fold(0, |acc, (result, operands)| {
                if can_produce_value(*result, operands) {
                    acc + result
                } else {
                    acc
                }
            });
            Ok(result)
        }
        Err(_err) => {
            dbg!(_err);
            Err("parsing failed".to_string())
        }
    }
}

fn can_produce_value(target_value: u64, operands: &Vec<u64>) -> bool {
    let num_operations = operands.len() - 1;
    let num_possible_combinations = 2u64.pow(num_operations as u32);

    for i in 0..num_possible_combinations {
        let mut operators = format!("{:016b}", i)
            .chars()
            .map(|c| {
                if c == '0' {
                    Operator::Add
                } else {
                    Operator::Mult
                }
            })
            .collect::<Vec<Operator>>();

        operators.push(Operator::Add); // First operator should be applied to the second operand
        operators.reverse();

        let operations = operands.iter().zip(operators.iter()).collect::<Vec<_>>();

        let result = operations[1..]
            .into_iter()
            .fold(operands[0], |acc, (operand, operator)| match operator {
                Operator::Add => acc + *operand,
                Operator::Mult => acc * *operand,
            });
        dbg!(target_value, &result);

        if result == target_value {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline, equation)(input)
}

fn equation(input: &str) -> IResult<&str, Equation> {
    let (next, result_value) = parse_number(input)?;
    let (next, _) = tag(": ")(next)?;
    let (input, inputs) = separated_list1(space1, parse_number)(next)?;
    let result = (result_value, inputs);
    Ok((input, result))
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    let (next, result) = digit1(input)?;
    Ok((next, result.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!(3749, process(contents)?);
        Ok(())
    }
}
