use std::error::Error;
use std::cmp::Ordering;

use crate::util::{read_file, to_u32};

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day4.txt")?.split(|&c| c == b'\n') {
        if line.len() == 0 {
            break;
        }
        let mut numbers = [0, 0];
        let mut flag = Ordering::Equal;
        for elf in line.split(|&c| c == b',') {
            for (i, number) in elf.split(|&c| c == b'-').enumerate() {
                if numbers[i] == 0 {
                    numbers[i] = to_u32(number);
                } else {
                    if i % 2 == 0 {
                        flag = numbers[i].cmp(&to_u32(number));
                    } else {
                        match numbers[i].cmp(&to_u32(number)) {
                            Ordering::Less => match flag {
                                Ordering::Less => (),
                                _ => score += 1,
                            },
                            Ordering::Equal => score += 1,
                            Ordering::Greater => match flag {
                                Ordering::Greater => (),
                                _ => score += 1,
                            },
                        }
                    }
                }
            }
        }
    }
    Ok(score)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day4.txt")?.split(|&c| c == b'\n') {
        if line.len() == 0 {
            break;
        }

        let numbers = line
            .split(|&c| c == b',' || c == b'-')
            .map(|n| to_u32(n))
            .collect::<Vec<u32>>();

        if numbers[2] <= numbers[1] && numbers[3] >= numbers[0] {
            score += 1;
        }
    }
    Ok(score)
}
