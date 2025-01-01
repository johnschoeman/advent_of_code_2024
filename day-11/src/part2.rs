use std::collections::HashMap;
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

type Stone = u64;
type Stones = Vec<u64>;
enum NextStones {
    Single(Stone),
    Double(Stone, Stone),
}

type Iteration = usize;
type Value = u64;
type StoneIteration = (Stone, Iteration);
type CountCache = HashMap<StoneIteration, Value>;

fn process(input: &str) -> Result<u64, String> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Stones>();

    let mut count_cache: CountCache = HashMap::new();
    count_cache.insert((0, 1), 1);
    count_cache.insert((0, 0), 1);
    count_cache.insert((1, 0), 1);
    count_cache.insert((1, 1), 1);
    count_cache.insert((2024, 0), 1);
    count_cache.insert((2024, 1), 2);

    let iterations = 75;

    let result = stones.iter().fold(0, |acc, stone| {
        let stone_iteration: StoneIteration = (*stone, iterations);
        let result = calculate_value(stone_iteration, &mut count_cache);
        acc + result
    });

    Ok(result)
}

fn calculate_value(stone_iteration: StoneIteration, count_cache: &mut CountCache) -> Value {
    // println!("stone: {} iteration: {}", stone_iteration.0, stone_iteration.1);
    // dbg!(&count_cache);
    if count_cache.contains_key(&stone_iteration) {
        let value = count_cache.get(&stone_iteration).unwrap();
        // println!("found: {:?} -> {}", &stone_iteration, value);
        return *value;
    }

    let (stone, iteration) = stone_iteration;

    if iteration == 0usize {
        // println!("No more iterations: {:?}", &stone_iteration);
        count_cache.insert(stone_iteration, 1);
        return 1;
    }

    let next_stones = process_stone(&stone);

    match next_stones {
        NextStones::Single(next_stone) => {
            let next_stone_iteration = (next_stone, iteration - 1);
            let next_value = calculate_value(next_stone_iteration, count_cache);

            count_cache.insert(stone_iteration, next_value);

            return next_value;
        }
        NextStones::Double(left_stone, right_stone) => {
            let left_stone_iteration = (left_stone, iteration - 1);
            let left_value = calculate_value(left_stone_iteration, count_cache);

            let right_stone_iteration = (right_stone, iteration - 1);
            let right_value = calculate_value(right_stone_iteration, count_cache);

            let next_value = left_value + right_value;

            count_cache.insert(stone_iteration, next_value);

            return next_value;
        }
    }
}

fn process_stone(stone: &Stone) -> NextStones {
    if *stone == 0 {
        return NextStones::Single(1);
    }

    let stone_str = format!("{}", stone);
    let stone_len = stone_str.len();
    if stone_len % 2 == 0 {
        let left = (&stone_str[..stone_len / 2]).parse::<u64>().unwrap();
        let right = (remove_leading_zeros(&stone_str[stone_len / 2..]))
            .parse::<u64>()
            .unwrap();

        return NextStones::Double(left, right);
    }

    let next = stone * 2024;
    return NextStones::Single(next);
}

fn remove_leading_zeros(input: &str) -> &str {
    let result = input.trim_start_matches('0');
    if result.is_empty() {
        return "0";
    } else {
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "125 17";
        assert_eq!(65601038650482, process(contents)?);
        Ok(())
    }
}
