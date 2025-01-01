use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input.txt";

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

type Stone = String;
type Stones = Vec<Stone>;

fn process(input: &str) -> Result<u32, String> {
    let mut stones = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Stones>();

    for x in 0..25 {
        // print_stones(&stones);
        println!("{} {}", x, stones.len());
        stones = blink(stones);
    }

    Ok(stones.len() as u32)
}

fn blink(stones: Stones) -> Stones {
    stones
        .iter()
        .flat_map(|stone| process_stone(stone.clone()))
        .collect()
}

fn process_stone(stone: Stone) -> Stones {
    if stone == "0" {
        return vec!["1".to_string()];
    }

    if stone.len() % 2 == 0 {
        let left = &stone[..stone.len() / 2];
        let right = remove_leading_zeros(&stone[stone.len() / 2..]);

        return vec![left.to_string(), right.to_string()];
    }

    let next = (stone.parse::<u64>().unwrap() * 2024).to_string();

    vec![next]
}

fn remove_leading_zeros(input: &str) -> String {
    let result = input.trim_start_matches('0').to_string();
    if result.is_empty() {
        return "0".to_string();
    } else {
        return result;
    }
}

fn print_stones(stones: &Stones) {
    let stones = stones
        .iter()
        .map(|stone| stone.to_string())
        .collect::<Vec<String>>();
    println!("{}", stones.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "125 17";
        assert_eq!(55312, process(contents)?);
        Ok(())
    }
}
