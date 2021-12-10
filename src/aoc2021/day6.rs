use std::error::Error;

use super::util::read_file;

fn parse_inputs(file_path: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let result = Vec::new();
    for line in read_file(file_path)?.lines() {
        return Ok(line
            .trim_end()
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect());
    }
    Ok(result)
}

fn solution(number_of_days: usize) -> Result<u64, Box<dyn Error>> {
    let inputs = parse_inputs("days/2021/day6.txt")?;
    let mut fishes = [0; 10];
    for i in inputs {
        fishes[i] += 1;
    }

    for _ in 0..number_of_days {
        fishes[7] += fishes[0];
        fishes[9] = fishes[0];
        for i in 1..10 {
            fishes[i - 1] = fishes[i];
        }
        fishes[9] = 0;
    }
    Ok(fishes.iter().sum())
}

pub fn first() -> Result<u64, Box<dyn Error>> {
    solution(80)
}

pub fn second() -> Result<u64, Box<dyn Error>> {
    solution(256)
}
