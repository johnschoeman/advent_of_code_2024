mod part_1;

fn main() {
    let result1 = part_1::run();
    match result1 {
        Ok(sum) => println!("Part 1 Result: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
