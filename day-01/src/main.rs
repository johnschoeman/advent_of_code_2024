mod day_1;

fn main() {
    println!("Day 1");
    match day_1::run() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
