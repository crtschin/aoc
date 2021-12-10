use std::error::Error;

use crate::aoc2021::util::read_file;

fn parse_inputs(file_path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in read_file(file_path)?.lines() {
        result.push(line.trim().bytes().collect());
    }
    Ok(result)
}

pub fn check_line(line: &Vec<u8>) -> Option<Result<Vec<u8>, u32>> {
    let mut stack: Vec<u8> = Vec::new();
    for &c in line {
        match c {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b'<' => stack.push(b'>'),
            _ => {
                if Some(c) != stack.pop() {
                    match c {
                        b')' => return Some(Err(3)),
                        b']' => return Some(Err(57)),
                        b'}' => return Some(Err(1197)),
                        b'>' => return Some(Err(25137)),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    if stack.len() > 0 {
        stack.reverse();
        Some(Ok(stack))
    } else {
        None
    }
}

pub fn first() -> Result<u32, Box<dyn Error>> {
    let parsed = parse_inputs("days/2021/day10.txt")?;
    let mut sum = 0;
    for line in parsed.iter() {
        match check_line(line) {
            Some(Err(score)) => sum += score,
            _ => {}
        }
    }
    Ok(sum)
}

pub fn second() -> Result<u64, Box<dyn Error>> {
    let parsed = parse_inputs("days/2021/day10.txt")?;
    let mut scores = Vec::with_capacity(parsed.len());
    for line in parsed.iter() {
        match check_line(line) {
            Some(Ok(missing)) => {
                scores.push(missing.iter().fold(0, |score, cur| {
                    score * 5
                        + match cur {
                            b')' => 1,
                            b']' => 2,
                            b'}' => 3,
                            b'>' => 4,
                            _ => unreachable!(),
                        }
                }));
            }
            _ => {}
        }
    }
    let which = scores.len() / 2;
    scores.select_nth_unstable(which);
    Ok(scores[which])
}
