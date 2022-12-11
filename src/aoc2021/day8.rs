use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

use crate::util::read_file_string;

struct Entry {
    signals: Vec<u8>,
    output: Vec<u8>,
}

fn entry(input: &str) -> nom::IResult<&str, Vec<Entry>> {
    let create_bitmap = |raw: Vec<&str>| {
        let mut signals: Vec<u8> = Vec::with_capacity(10);
        for signal in raw.iter() {
            let overlay = signal
                .chars()
                .fold(0, |acc, c| acc | (1 << (c as usize - 'a' as usize)));
            signals.push(overlay);
        }
        signals
    };
    let create = |raw: (Vec<u8>, Vec<u8>)| Entry {
        signals: raw.0,
        output: raw.1,
    };
    separated_list1(
        newline,
        map(
            separated_pair(
                map(separated_list1(tag(" "), alpha1), create_bitmap),
                tag(" | "),
                map(separated_list1(tag(" "), alpha1), create_bitmap),
            ),
            create,
        ),
    )(input)
}

fn parse_inputs(file_path: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let parse_entry = |s| -> Vec<Entry> {
        let (_, ranges) = entry(s).unwrap();
        ranges
    };
    let raw = read_file_string(file_path)?;
    let parsed = parse_entry(&raw);
    Ok(parsed)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    let entries = parse_inputs("days/2021/day8.txt")?;
    Ok(entries
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|s| {
                    let ones = s.count_ones();
                    (1 < ones && ones < 5) || ones == 7
                })
                .count() as i32
        })
        .sum::<i32>())
}
pub fn second() -> Result<i32, Box<dyn Error>> {
    let entries = parse_inputs("days/2021/day8.txt")?;
    let mut sum = 0;
    for entry in entries {
        let mut lengths = vec![Vec::new(); 8];
        for signal in entry.signals {
            lengths[signal.count_ones() as usize].push(signal);
        }
        let mut number = 0;
        for output in entry.output {
            match output.count_ones() {
                2 => number = number * 10 + 1,
                3 => number = number * 10 + 7,
                4 => number = number * 10 + 4,
                5 => {
                    if (output & !&lengths[2][0]).count_ones() == 3 {
                        number = number * 10 + 3
                    } else if (output & !&lengths[4][0]).count_ones() == 2 {
                        number = number * 10 + 5
                    } else {
                        number = number * 10 + 2
                    }
                }
                6 => {
                    if (output & !&lengths[2][0]).count_ones() == 5 {
                        number = number * 10 + 6
                    } else if (output & !&lengths[4][0]).count_ones() == 2 {
                        number = number * 10 + 9
                    } else {
                        number = number * 10 + 0
                    }
                }
                7 => number = number * 10 + 8,
                _ => panic!(),
            }
        }
        sum += number;
    }
    Ok(sum)
}
