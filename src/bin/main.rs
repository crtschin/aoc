use aoc::aoc2021::*;

fn main() {
    let start = std::time::Instant::now();
    match day4::second() {
        Ok(result) => {
            let end = std::time::Instant::now();
            println!("Time: {:?}", end - start);
            println!("Result: {}", result);
        },
        Err(e) => println!("Failed: {}", e),
    }
}
