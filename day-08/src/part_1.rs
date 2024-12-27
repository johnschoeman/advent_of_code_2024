use itertools::Itertools;
use nom::{
    bytes::complete::{take, tag},
    branch::alt,
    multi::{many0, many1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

type Span<'a> = LocatedSpan<&'a str>;
type Frequency<'a> = &'a str;
type Position<'a> = (i32, i32);
type AntennaMap<'a> = HashMap<Frequency<'a>, Vec<Position<'a>>>;

const FILE_PATH: &str = "./input.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<u32, String> {
    let input_rows = input.lines().count() as i32;
    let input_cols = input.lines().next().unwrap().len() as i32;
    match parse(Span::new(input)) {
        Ok((_remaining, annetena_map)) => {
            let mut annodes: HashSet<Position<'_>> = HashSet::new();
            for (_frequency, positions) in annetena_map {
                let node_pairs = positions.into_iter().combinations(2).collect::<Vec<_>>();

                for pairs in node_pairs {
                    let posistions = node_positions(&pairs[0], &pairs[1]);
                    let nodes = posistions
                        .into_iter()
                        .filter(|(row, col)| {
                            *row >= 0 && *col >= 0 && *row < input_rows && *col < input_cols
                        })
                        .collect::<Vec<_>>();

                    for node in nodes {
                        annodes.insert(node);
                    }
                }
            }

            show_annodes(input_cols, input_rows, &annodes);
            // dbg!(&annodes);
            let result = annodes.len() as u32;
            Ok(result)
        }
        Err(_err) => {
            dbg!(_err);
            Err("parsing failed".to_string())
        }
    }
}

fn node_positions<'a>(
    lower_antenna: &Position<'a>,
    higher_antenna: &Position<'a>,
) -> Vec<Position<'a>> {
    let (lower_row, lower_col) = lower_antenna;
    let (higher_row, higher_col) = higher_antenna;

    let rise = higher_row - lower_row;
    let run = higher_col - lower_col;

    let node_a = (lower_row - rise, lower_col - run);
    let node_b = (higher_row + rise, higher_col + run);

    [node_a, node_b].to_vec()
}

fn parse(input: Span) -> IResult<Span, AntennaMap> {
    let (next, antennas) = many1(antenna)(input)?;

    let mut antenna_map = HashMap::new();
    for (frequency, position) in antennas {
        antenna_map
            .entry(frequency)
            .or_insert_with(Vec::new)
            .push(position.clone());
    }

    let (next, _) = many0(alt((tag("."), tag("\n"))))(next)?;
    Ok((next, antenna_map))
}

fn antenna(input: Span) -> IResult<Span, (&str, (i32, i32))> {
    let (next, _) = many0(alt((tag("."), tag("\n"))))(input)?;

    let row = next.location_line();
    let col = next.get_column();
    let (next, antenna) = take(1usize)(next)?;

    let antenna_str = *antenna.fragment();
    Ok((next, (antenna_str, (row as i32 - 1, col as i32 - 1))))
}

fn show_annodes(width: i32, height: i32, annodes: &HashSet<(i32, i32)>) {
    for row in 0..height {
        for col in 0..width {
            if annodes.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(14, process(contents)?);
        Ok(())
    }
}

// ......#....#
// ...#....0...
// ....#0....#.
// ..#....0....
// ....0....#..
// .#....A.....
// ...#........
// #......#....
// ........A...
// .........A..
// ..........#.
// ..........#.

// ......#....#
// ...#....0...
// ....#0....#.
// ..#....0....
// ....0....#..
// .#...#A.....
// ...#........
// #......#....
// ........A...
// .........A..
// ..........#.
// ..........#.
