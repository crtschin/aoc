use std::error::Error;

use crate::util::read_file;

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day2.txt")?.split(|&c| c == b'\n') {
        let split: (&[u8], &[u8]) = line.split_at(1);
        match split.1[1] {
            b'X' => {
                match split.0[0] {
                    b'A' => score += 3,
                    b'B' => score += 0,
                    b'C' => score += 6,
                    _ => panic!("Invalid input"),
                }
                score += 1
            }
            b'Y' => {
                match split.0[0] {
                    b'A' => score += 6,
                    b'B' => score += 3,
                    b'C' => score += 0,
                    _ => panic!("Invalid input"),
                }
                score += 2
            }
            b'Z' => {
                match split.0[0] {
                    b'A' => score += 0,
                    b'B' => score += 6,
                    b'C' => score += 3,
                    _ => panic!("Invalid input"),
                }
                score += 3
            }
            _ => panic!("Invalid input"),
        }
    }
    Ok(score)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day2.txt")?.split(|&c| c == b'\n') {
        if line.len() == 0 {
            continue;
        }

        let split: (&[u8], &[u8]) = line.split_at(1);
        match split.1[1] {
            b'X' => {
                match split.0[0] {
                    b'A' => score += 3,
                    b'B' => score += 1,
                    b'C' => score += 2,
                    _ => panic!("Invalid input"),
                }
                score += 0
            }
            b'Y' => {
                match split.0[0] {
                    b'A' => score += 1,
                    b'B' => score += 2,
                    b'C' => score += 3,
                    _ => panic!("Invalid input"),
                }
                score += 3
            }
            b'Z' => {
                match split.0[0] {
                    b'A' => score += 2,
                    b'B' => score += 3,
                    b'C' => score += 1,
                    _ => panic!("Invalid input"),
                }
                score += 6
            }
            _ => panic!("Invalid input"),
        }
    }
    Ok(score)
}
