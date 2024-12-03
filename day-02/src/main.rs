mod part_1;

fn main() {
    let result = part_1::run();
    match result {
        Ok(sum) => println!("Part 1 Result: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
