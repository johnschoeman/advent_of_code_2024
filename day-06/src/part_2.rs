use itertools::Itertools;
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

fn show_position(pos: &Position) -> &str {
    match pos {
        Position::Empty => ".",
        Position::Obsruction => "#",
        Position::Guard(Direction::Up) => "^",
        Position::Guard(Direction::Down) => "v",
        Position::Guard(Direction::Left) => "<",
        Position::Guard(Direction::Right) => ">",
        Position::Path => "X",
    }
}

type Lab = Vec<Vec<Position>>;

fn print_lab(lab: &Lab) -> () {
    for row in lab.iter() {
        let line_str = row.iter().map(|pos| show_position(pos)).join("");
        println!("{}", line_str);
    }

    ()
}

type Coord = (usize, usize);
type PathHistory = Vec<(Coord, Direction)>;

#[derive(Debug)]
enum CompleteRoute {
    Exited(Lab, PathHistory),
    Looped(Lab, PathHistory),
}

#[derive(Debug)]
enum GuardRoute {
    Incomplete(Lab, PathHistory),
    Complete(CompleteRoute),
}

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<usize, String> {
    match parse(input) {
        Ok((_remaining, initial_lab)) => {
            // dbg!(&initial_lab);

            let (lab, initial_path) = match evaluate_guard_route(&initial_lab) {
                CompleteRoute::Exited(lab, path) => (lab, path),
                CompleteRoute::Looped(lab, path) => (lab, path),
            };
            // dbg!(&lab);
            // print_lab(&lab);

            let coords = initial_path[1..]
                .iter()
                .map(|(coord, _dir)| coord)
                .unique()
                .collect::<Vec<_>>();

            let total_coords = coords.len();
            let mut count = 0;

            let result = coords
                .into_iter()
                .filter(|(row, col)| {
                    // dbg!((row, col));
                    dbg!(count, total_coords, count as f64 / total_coords as f64);
                    count += 1;
                    let mut lab_with_obstruction = initial_lab.clone();
                    lab_with_obstruction[*row][*col] = Position::Obsruction;
                    match evaluate_guard_route(&lab_with_obstruction) {
                        CompleteRoute::Exited(_, _) => return false,
                        CompleteRoute::Looped(_lab, _path) => {
                            // dbg!(&lab);
                            return true;
                        }
                    }
                })
                .count();
            Ok(result)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn evaluate_guard_route(lab: &Lab) -> CompleteRoute {
    let mut next_route = walk(lab, vec![]);
    let mut count = 0;

    loop {
        match next_route {
            GuardRoute::Incomplete(lab, history) => {
                count += 1;
                if count > 10000 {
                    // dbg!(&lab);
                    // dbg!(&history);
                    return CompleteRoute::Looped(lab, history.clone());
                }
                next_route = walk(&lab, history);
            }
            GuardRoute::Complete(route) => return route,
        }
    }
}

fn walk(lab: &Lab, mut path_history: PathHistory) -> GuardRoute {
    let mut next_lab = lab.clone();
    let ((guard_row, guard_col), guard_dir) = match find_guard(&lab) {
        Some(guard) => guard,
        None => return GuardRoute::Complete(CompleteRoute::Exited(next_lab, path_history)),
    };

    match guard_dir {
        Direction::Up => {
            if guard_row == 0 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(CompleteRoute::Exited(next_lab, path_history));
            }
        }
        Direction::Down => {
            if guard_row == lab.len() - 1 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(CompleteRoute::Exited(next_lab, path_history));
            }
        }
        Direction::Left => {
            if guard_col == 0 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(CompleteRoute::Exited(next_lab, path_history));
            }
        }
        Direction::Right => {
            if guard_col == lab[0].len() - 1 {
                next_lab[guard_row][guard_col] = Position::Path;
                return GuardRoute::Complete(CompleteRoute::Exited(next_lab, path_history));
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
            let next_pos = match guard_dir {
                Direction::Up | Direction::Down => Position::Path,
                Direction::Left | Direction::Right => Position::Path,
            };
            next_lab[guard_row][guard_col] = next_pos;
        }
        Some(next_pos) => match next_pos {
            Position::Empty => {
                let next_pos = match guard_dir {
                    Direction::Up | Direction::Down => Position::Path,
                    Direction::Left | Direction::Right => Position::Path,
                };
                next_lab[guard_row][guard_col] = next_pos;
                next_lab[next_row][next_col] = Position::Guard(guard_dir.clone());
            }
            Position::Path => {
                if path_history.contains(&((next_row, next_col), guard_dir.clone())) {
                    return GuardRoute::Complete(CompleteRoute::Looped(next_lab, path_history));
                }
                let next_pos = Position::Path;
                next_lab[guard_row][guard_col] = next_pos;
                next_lab[next_row][next_col] = Position::Guard(guard_dir.clone());
            }
            Position::Obsruction => {
                next_lab[guard_row][guard_col] = Position::Guard(rotate_dir(guard_dir));
            }
            _ => panic!("Invalid position"),
        },
    }

    path_history.push(((guard_row, guard_col), guard_dir.clone()));
    GuardRoute::Incomplete(next_lab, path_history)
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
        assert_eq!(6, process(contents)?);
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
