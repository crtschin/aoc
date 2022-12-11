use std::error::Error;
use std::cmp::Ordering;
use std::str;

use crate::util::read_file;

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day4.txt")?.split(|&c| c == b'\n') {
        if line.len() == 0 {
            break;
        }
        let mut numbers = [-1, -1];
        let mut flag = Ordering::Equal;
        for elf in line.split(|&c| c == b',') {
            for (i, number) in elf.split(|&c| c == b'-').enumerate() {
                if numbers[i] == -1 {
                    numbers[i] = str::from_utf8(number)?.parse::<i32>()?;
                } else {
                    if i % 2 == 0 {
                        flag = numbers[i].cmp(&str::from_utf8(number)?.parse::<i32>().unwrap());
                    } else {
                        match numbers[i].cmp(&str::from_utf8(number)?.parse::<i32>().unwrap()) {
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
            .map(|n| str::from_utf8(n).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if numbers[2] <= numbers[1] && numbers[3] >= numbers[0] {
            score += 1;
        }
    }
    Ok(score)
}
