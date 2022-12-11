use std::error::Error;
use std::str;

use crate::util::read_file;

fn parse_inputs() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current_vec = Vec::new();
    for line in read_file("days/2022/day1.txt")?.split(|&c| c == b'\n') {
        if line.len() > 0 {
            current_vec.push(str::from_utf8(line).unwrap().parse::<i32>().unwrap());
        } else {
            result.push(current_vec);
            current_vec = Vec::new();
        }
    }
    Ok(result)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    let result: i32 = parse_inputs()?
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap();
    Ok(result)
}

pub fn first_() -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    let mut max = 0;
    for line in read_file("days/2022/day1.txt")?.split(|&c| c == b'\n') {
        if line.len() > 0 {
            sum += str::from_utf8(line).unwrap().parse::<i32>().unwrap();
        } else {
            max = max.max(sum);
            sum = 0;
        }
    }
    Ok(max)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let mut result: Vec<i32> = parse_inputs()?.iter().map(|elf| elf.iter().sum()).collect();
    let length = result.len();
    let subset = result.select_nth_unstable(length - 4);
    Ok(subset.2.iter().sum())
}
