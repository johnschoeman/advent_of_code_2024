mod part_1;
mod part_2;

fn main() {
    // let result1 = part_1::run();
    // match result1 {
    // Ok(sum) => println!("Part 1 Result: {}", sum),
    // Err(e) => println!("Error: {}", e),
    // }
    let result2 = part_2::run();
    match result2 {
        Ok(sum) => println!("Part 2 Result: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
