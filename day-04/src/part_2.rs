use nom::{
    bytes::complete::is_not, character::complete::newline, combinator::map, multi::separated_list1,
    IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => {
            // dbg!(&result);
            Ok(result.to_string())
        }
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<u32, String> {
    match parse(input) {
        Ok((_, matrix)) => {
            let sum = count_xmases(&matrix);

            return Ok(sum);
        }
        Err(_err) => return Err("failed to parse".to_string()),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(
        newline,
        map(is_not("\n"), |matched: &str| {
            matched.split("").filter(|&x| x != "").collect()
        }),
    )(input)
}

fn count_xmases(m: &Vec<Vec<&str>>) -> u32 {
    (0..m.len() - 2)
        .map(|row| {
            let count = (0..m[0].len() - 2)
                .filter(move |col| {
                    let l_diag = [m[row][*col], m[row + 1][col + 1], m[row + 2][col + 2]].to_vec();
                    let r_diag = [m[row][col + 2], m[row + 1][col + 1], m[row + 2][*col]].to_vec();
                    // dbg!(&l_diag);
                    // dbg!(&r_diag);
                    is_mas(&l_diag) && is_mas(&r_diag)
                })
                .count();
            return count;
        })
        .sum::<usize>() as u32
}

fn is_mas(line: &Vec<&str>) -> bool {
    *line == ["M", "A", "S"].to_vec() || *line == ["S", "A", "M"].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, process(contents)?);
        Ok(())
    }
}
