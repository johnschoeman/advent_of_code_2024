use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::value,
    multi::many0, IResult,
};
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Empty,
    Obsruction,
    Guard(Direction),
    Path,
}

type Lab = Vec<Vec<Position>>;

#[derive(Debug)]
enum GuardRoute {
    Incomplete(Lab),
    Complete(Lab),
}

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<u32, String> {
    match parse(input) {
        Ok((_remaining, initial_lab)) => {
            // dbg!(&initial_lab);
            let mut next_lab = walk(initial_lab);
            loop {
                match next_lab {
                    GuardRoute::Incomplete(lab) => {
                        next_lab = walk(lab);
                    }
                    GuardRoute::Complete(lab) => {
                        // dbg!(&lab);
                        let path_length = lab
                            .iter()
                            .flatten()
                            .filter(|&pos| *pos == Position::Path)
                            .count();
                        return Ok(path_length as u32);
                    }
                }
            }
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn walk(lab: Lab) -> GuardRoute {
    let mut next_lab = lab.clone();
    let ((guard_row, guard_col), guard_dir) = match find_guard(&lab) {
        Some(guard) => guard,
        None => return GuardRoute::Complete(lab),
    };

    match guard_dir {
        Direction::Up => {
            if guard_row == 0 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(next_lab);
            }
        }
        Direction::Down => {
            if guard_row == lab.len() - 1 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(next_lab);
            }
        }
        Direction::Left => {
            if guard_col == 0 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(next_lab);
            }
        }
        Direction::Right => {
            if guard_col == lab[0].len() - 1 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(next_lab);
            }
        }
    }

    let (next_row, next_col) = match guard_dir {
        Direction::Up => (guard_row - 1, guard_col),
        Direction::Down => (guard_row + 1, guard_col),
        Direction::Left => (guard_row, guard_col - 1),
        Direction::Right => (guard_row, guard_col + 1),
    };

    let option_next_pos = &lab.get(next_row).and_then(|row| row.get(next_col));

    match option_next_pos {
        None => {
            next_lab[guard_row][guard_col] = Position::Path;
        }
        Some(next_pos) => match next_pos {
            Position::Empty | Position::Path => {
                next_lab[guard_row][guard_col] = Position::Path;
                next_lab[next_row][next_col] = Position::Guard(guard_dir.clone());
            }
            Position::Obsruction => {
                next_lab[guard_row][guard_col] = Position::Guard(rotate_dir(guard_dir));
            }
            _ => panic!("Invalid position"),
        },
    }

    GuardRoute::Incomplete(next_lab)
}

fn rotate_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn find_guard(lab: &Lab) -> Option<((usize, usize), &Direction)> {
    for (i, row) in lab.iter().enumerate() {
        for (j, pos) in row.iter().enumerate() {
            if let Position::Guard(direction) = pos {
                return Some(((i, j), direction));
            }
        }
    }
    None
}

fn parse(input: &str) -> IResult<&str, Lab> {
    nom::multi::separated_list0(newline, many0(parse_lab_space))(input)
}

fn parse_lab_space(input: &str) -> IResult<&str, Position> {
    alt((
        value(Position::Empty, tag(".")),
        value(Position::Obsruction, tag("#")),
        value(Position::Guard(Direction::Up), tag("^")),
        value(Position::Guard(Direction::Down), tag("v")),
        value(Position::Guard(Direction::Left), tag("<")),
        value(Position::Guard(Direction::Right), tag(">")),
        value(Position::Path, tag("X")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(41, process(contents)?);
        Ok(())
    }
}

// ....#.....
// ....XXXXX#
// ....X...X.
// ..#.X...X.
// ..XXXXX#X.
// ..X.X.X.X.
// .#XXXXXXX.
// .XXXXXXX#.
// #XXXXXXX..
// ......#X..
