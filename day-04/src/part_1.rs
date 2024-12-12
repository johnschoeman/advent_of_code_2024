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
        Ok((_, rows)) => {
            let cols = transpose(&rows);
            let left_diags = diagonals(&rows);
            let right_diags = diagonals_left(&rows);

            // dbg!(&rows);
            // dbg!(&cols);
            // dbg!(&left_diags);
            // dbg!(&right_diags);

            let sum = [rows, cols, left_diags, right_diags]
                .iter()
                .map(|v| count_xmases(v))
                .sum::<u32>();

            return Ok(sum);
        }
        Err(_err) => return Err("failed to parse".to_string()),
    }
}

fn count_xmases(v: &Vec<Vec<&str>>) -> u32 {
    v.iter().map(count_xmas).sum::<u32>()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(
        newline,
        map(is_not("\n"), |matched: &str| {
            matched.split("").filter(|&x| x != "").collect()
        }),
    )(input)
}

fn count_xmas(line: &Vec<&str>) -> u32 {
    line.windows(4)
        .filter(|&w| w == ["X", "M", "A", "S"] || w == ["S", "A", "M", "X"])
        .count() as u32
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn diagonals<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut result = Vec::new();
    let mut row_a = 0;
    let mut col_a = v[0].len() - 1;
    // start at top right, go left then down
    while row_a < v.len() {
        let mut diag = Vec::new();
        let mut row_x = row_a;
        let mut col_x = col_a;
        while row_x < v.len() && col_x < v[0].len() {
            diag.push(v[row_x][col_x].clone());
            row_x += 1;
            col_x += 1;
        }
        result.push(diag);
        if col_a > 0 {
            col_a -= 1;
        } else {
            row_a += 1;
        }
    }
    result
}

fn diagonals_left<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut result = Vec::new();
    let mut row_a = v.len() - 1;
    let mut col_a = v[0].len() - 1;
    // start a bottom right, go up then left
    while col_a > 0 {
        let mut diag = Vec::new();
        let mut row_x = row_a;
        let mut col_x = col_a;

        diag.push(v[row_x][col_x].clone());
        while row_x < v.len() - 1 && col_x > 0 {
            row_x += 1;
            col_x -= 1;
            diag.push(v[row_x][col_x].clone());
        }
        result.push(diag);

        if row_a > 0 {
            row_a -= 1;
        } else {
            col_a -= 1;
        }
    }
    result
}

type Matrix<'a> = Vec<Vec<&'a str>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diagonals() -> Result<(), String> {
        let input: Matrix = [["A", "B"].to_vec(), ["C", "D"].to_vec()].to_vec();

        let result = diagonals(&input);

        let expected: Matrix = [["B"].to_vec(), ["A", "D"].to_vec(), ["C"].to_vec()].to_vec();

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_count_xmas() -> Result<(), String> {
        let line = ["X", "M", "A", "S", "X", "X", "X", "S", "A", "M", "X", "A"].to_vec();
        let result = count_xmas(&line);
        assert_eq!(2, result);
        Ok(())
    }

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
        assert_eq!(18, process(contents)?);
        Ok(())
    }

    // #[test]
    // fn test_process_2() -> Result<(), String> {
    //     let contents = "XSXMAAXXSSMMMXMXSXMSXMXSAMXSXMASMMSSMMSASXSAAXAAMXMMAMAMXMXSMXSAMXAXSAMXSSSXMASAMXAAMXSXMASAMXXMAXXSAXAMXMMSAASMXMXMASMMAMXXXSAMMSMMSXMASXAA";
    //     assert_eq!(10, process(contents)?);
    //     Ok(())
    // }

    #[test]
    fn test_process_3() -> Result<(), String> {
        let contents = "\
XSXMAA
XXAMMS
SSSMAA
XAAMSS
MSMMAX
XMASMM";
        assert_eq!(2, process(contents)?);
        Ok(())
    }
}
