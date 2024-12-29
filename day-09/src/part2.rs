use std::collections::HashMap;
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    // dbg!(&contents);

    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

type Id = usize;
type Length = usize;

#[derive(Debug, Clone)]
enum Block {
    File(Id, Length),
    Free(Length),
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize, String> {
    let mut current_id = 0;
    let mut file_map: HashMap<Id, (usize, Length)> = HashMap::new();
    let mut disk: Vec<Block> = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(idx, x)| {
            let count = x.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                let id = idx / 2;
                current_id = id;
                file_map.insert(id, (idx, count));
                return (1..=count).rev().map(|len| Block::File(id, len)).collect();
            } else {
                return (1..=count)
                    .rev()
                    .map(|len| Block::Free(len))
                    .collect::<Vec<_>>();
            }
        })
        .collect();

    // print_disk(&disk);

    while current_id > 1 {
        let space_needed = file_map.get(&current_id).unwrap().1;

        let option_move_to_idx = disk.iter().position(|x| match x {
            Block::File(_, _) => return false,
            Block::Free(len) => {
                return len >= &space_needed;
            }
        });

        let move_from_idx = disk
            .iter()
            .position(|x| match x {
                Block::File(id, _) => {
                    return *id == current_id;
                }
                Block::Free(_) => return false,
            })
            .unwrap();

        // print_disk(&disk);
        // dbg!(current_id, option_move_to_idx, move_from_idx);

        if option_move_to_idx.is_some() {
            let move_to_idx = option_move_to_idx.unwrap();
            if move_to_idx < move_from_idx {
                disk = move_file_on(disk, current_id, move_to_idx, move_from_idx, space_needed)
            }
        };

        current_id -= 1;
    }

    // dbg!(&disk);
    // print_disk(&disk);

    let result = disk
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, x)| match x {
            Block::File(id, _len) => {
                return acc + id * idx;
            }
            Block::Free(_len) => {
                return acc;
            }
        });

    return Ok(result);
}

fn move_file_on(
    mut disk: Vec<Block>,
    file_id: Id,
    move_to_idx: usize,
    move_from_idx: usize,
    len: usize,
) -> Vec<Block> {
    for i in 0..len {
        disk[move_to_idx + i] = Block::File(file_id, len - i);
        disk[move_from_idx + i] = Block::Free(len - i);
    }
    return disk;
}

fn print_disk(disk: &Vec<Block>) {
    for x in disk {
        match x {
            Block::File(id, _len) => {
                print!("{}", id);
            }
            Block::Free(_len) => {
                print!("-");
            }
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "12345";
        assert_eq!(132, process(contents)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> Result<(), String> {
        let contents = "2333133121414131402";
        assert_eq!(2858, process(contents)?);
        Ok(())
    }
}
