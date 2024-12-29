use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    dbg!(&contents);

    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

type Id = usize;

#[derive(Debug, Clone)]
enum Disk {
    FileBlock(Id),
    FreeSpace,
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize, String> {
    let mut disk: Vec<Disk> = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(idx, x)| {
            let count = x.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                let id = idx / 2;
                return vec![Disk::FileBlock(id); count];
            } else {
                return vec![Disk::FreeSpace; count];
            }
        })
        .collect();
    // dbg!(&unformatted_disk);

    let mut freeing_idx: usize = disk.len() - 1;
    let mut move_to_idx: usize = 0;

    while freeing_idx > move_to_idx {
        match disk[move_to_idx] {
            Disk::FileBlock(_) => {
                move_to_idx += 1;
            }
            Disk::FreeSpace => match disk[freeing_idx] {
                Disk::FileBlock(id) => {
                    disk[move_to_idx] = Disk::FileBlock(id);
                    disk[freeing_idx] = Disk::FreeSpace;
                    move_to_idx += 1;
                    freeing_idx -= 1;
                }
                Disk::FreeSpace => {
                    freeing_idx -= 1;
                }
            },
        }
    }

    // dbg!(&disk);
    // print_disk(&disk);

    let result = disk
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, x)| match x {
            Disk::FileBlock(id) => {
                return acc + id * idx;
            }
            Disk::FreeSpace => {
                return acc;
            }
        });

    return Ok(result);
}

fn print_disk(disk: &Vec<Disk>) {
    for x in disk {
        match x {
            Disk::FileBlock(id) => {
                print!("{}", id);
            }
            Disk::FreeSpace => {
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
        assert_eq!(60, process(contents)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> Result<(), String> {
        let contents = "2333133121414131402";
        assert_eq!(1928, process(contents)?);
        Ok(())
    }
}
