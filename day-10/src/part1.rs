use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::value,
    multi::{many0, separated_list0},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

type Position = (i32, i32);
type Elevation = i32;
type Topo = Vec<Vec<Elevation>>;

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
        Ok((_remaining, topo)) => {
            let height = topo.len() - 1;
            let width = topo[0].len() - 1;
            let heads = find_all_positins(&topo, 0);
            let peaks = find_all_positins(&topo, 9);

            let result: u32 = heads
                .into_iter()
                .map(|head| {
                    let mut initial_visited: HashSet<Position> = HashSet::new();
                    determine_trails(&head, &topo, &mut initial_visited, height, width)
                })
                .sum();

            // let mut visited: HashSet<Position> = HashSet::new();
            // let head = &heads[0];
            // dbg!(&head);
            // let result = determine_trails(head, &topo, &mut visited, height, width);

            Ok(result)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn determine_trails(
    head: &Position,
    topo: &Topo,
    mut visited: &mut HashSet<Position>,
    height: usize,
    width: usize,
) -> u32 {
    let head_elevation = topo[head.0 as usize][head.1 as usize];
    // dbg!(&head, &head_elevation);

    if head_elevation == 9 {
        return 1;
    }

    let binding = adjacent_positions(head);
    let next_positions = binding
        .iter()
        .filter(|pos| {
            let (row, col) = pos;
            *row >= 0 && *row < height as i32 && *col >= 0 && *col <= width as i32
        })
        .filter(|pos| {
            // !visited.contains(pos)
            true
        })
        .filter(|pos| {
            let (row_idx, col_idx) = pos;
            let row = &topo[*row_idx as usize];
            if row.len() == 0 {
                dbg!(&row_idx, &col_idx, &row, &height, &width);
            }
            let elevation = &row[*col_idx as usize];
            elevation - head_elevation == 1
        })
        .collect::<Vec<&Position>>();

    next_positions.iter().for_each(|&pos| {
        visited.insert(*pos);
    });

    // dbg!(&next_positions);

    next_positions.into_iter().fold(0, |acc, pos| {
        acc + determine_trails(pos, topo, &mut visited, height, width)
    })
}

fn adjacent_positions(pos: &Position) -> Vec<Position> {
    let left: Position = (pos.0 - 1, pos.1);
    let up: Position = (pos.0, pos.1 + 1);
    let right: Position = (pos.0 + 1, pos.1);
    let down: Position = (pos.0, pos.1 - 1);

    vec![left, up, right, down]
}

fn find_all_positins(matrix: &Vec<Vec<Elevation>>, target: Elevation) -> Vec<Position> {
    let mut positions = Vec::new();
    for (row, row_values) in matrix.iter().enumerate() {
        for (col, value) in row_values.iter().enumerate() {
            if *value == target {
                positions.push((row as i32, col as i32));
            }
        }
    }
    positions
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Elevation>>> {
    separated_list0(newline, many0(parse_elevation))(input)
}

fn parse_elevation(input: &str) -> IResult<&str, Elevation> {
    alt((
        value(0, tag("0")),
        value(1, tag("1")),
        value(2, tag("2")),
        value(3, tag("3")),
        value(4, tag("4")),
        value(5, tag("5")),
        value(6, tag("6")),
        value(7, tag("7")),
        value(8, tag("8")),
        value(9, tag("9")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(36, process(contents)?);
        Ok(())
    }
}

// 89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
