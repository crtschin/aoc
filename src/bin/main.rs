use aoc::{
    aoc2021,
    aoc2022,
};

fn main() {
    let start = std::time::Instant::now();
    match aoc2022::day4::second() {
        Ok(result) => {
            let end = std::time::Instant::now();
            let time_diff = end - start;
            println!("Time: {:?}", time_diff);
            println!("Result: {:?}", result);
        },
        Err(e) => println!("Failed: {}", e)
    }
}
