use std::error::Error;

use crate::util::read_file_string;

enum Instr {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_inputs(file_path: &str) -> Result<Vec<Instr>, Box<dyn Error>> {
    let mut result: Vec<Instr> = Vec::new();
    for line in read_file_string(file_path)?.lines() {
        let split: Vec<&str> = line.trim_end().split(" ").collect();
        let amount: i32 = split[1].parse()?;
        match split[0] {
            "forward" => result.push(Instr::Forward(amount)),
            "down" => result.push(Instr::Down(amount)),
            "up" => result.push(Instr::Up(amount)),
            _ => (),
        }
    }
    Ok(result)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut depth = 0;
    let mut horizontal = 0;
    for instr in parse_inputs("days/2021/day2.txt")? {
        match instr {
            Instr::Forward(amount) => horizontal += amount,
            Instr::Up(amount) => depth -= amount,
            Instr::Down(amount) => depth += amount,
        }
    }
    Ok(depth * horizontal)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for instr in parse_inputs("days/2021/day2.txt")? {
        match instr {
            Instr::Forward(amount) => {
                depth += aim * amount;
                horizontal += amount;
            }
            Instr::Up(amount) => aim -= amount,
            Instr::Down(amount) => aim += amount,
        }
    }
    Ok(depth * horizontal)
}
