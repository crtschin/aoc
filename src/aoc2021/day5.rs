use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{separated_pair, terminated};

use crate::util::read_file_string;

struct Range {
    from: (i32, i32),
    to: (i32, i32),
}

fn decimal(input: &str) -> nom::IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn range(input: &str) -> nom::IResult<&str, Vec<Range>> {
    separated_list1(
        newline,
        map(
            separated_pair(
                separated_pair(decimal, char(','), decimal),
                tag(" -> "),
                separated_pair(decimal, char(','), decimal),
            ),
            |(from, to)| Range {
                from: (from.0.parse().unwrap(), from.1.parse().unwrap()),
                to: (to.0.parse().unwrap(), to.1.parse().unwrap()),
            },
        ),
    )(input)
}

fn parse_inputs(file_path: &str) -> Result<Vec<Range>, Box<dyn Error>> {
    let parse_range = |s| -> Vec<Range> {
        let (_, ranges) = range(s).unwrap();
        ranges
    };
    let raw = read_file_string(file_path)?;
    let parsed = parse_range(&raw);
    Ok(parsed)
}

#[derive(Clone, Copy)]
enum BoardLoc {
    Empty,
    First,
    Second,
}

const SIZE: usize = 1000;
static mut BOARD: [BoardLoc;SIZE*SIZE] = [BoardLoc::Empty; SIZE * SIZE];

fn process_ranges<'a, T: Iterator<Item = &'a Range>>(ranges: T) -> u16 {
    let mut result = 0;
    for range in ranges {
        let (x, y) = (range.to.0 - range.from.0, range.to.1 - range.from.1);
        let steps = x.abs().max(y.abs());
        let direction = (x.signum(), y.signum());
        let base = range.from.1 + range.from.0 * SIZE as i32;
        for i in 0..(steps + 1) {
            let key = (base + i * (direction.1 + direction.0 * SIZE as i32)) as usize;
            unsafe {
                match BOARD[key] {
                    BoardLoc::Empty => {
                        BOARD[key] = BoardLoc::First;
                    }
                    BoardLoc::First => {
                        BOARD[key] = BoardLoc::Second;
                        result += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    result
}

fn solution(diagonal: bool) -> Result<u16, Box<dyn Error>> {
    let ranges = parse_inputs("days/2021/day5.txt")?;
    if !diagonal {
        let result = process_ranges(
            ranges
                .iter()
                .filter(|&range| range.to.1 == range.from.1 || range.to.0 == range.from.0),
        );
        Ok(result)
    } else {
        let result = process_ranges(ranges.iter().filter(|&range| {
            range.to.1 == range.from.1
                || range.to.0 == range.from.0
                || (range.to.1 - range.from.1).abs() == (range.to.0 - range.from.0).abs()
        }));
        Ok(result)
    }
}

pub fn first() -> Result<u16, Box<dyn Error>> {
    solution(false)
}

pub fn second() -> Result<u16, Box<dyn Error>> {
    solution(true)
}
