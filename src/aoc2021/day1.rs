use std::error::Error;

use crate::util::read_file_string;

fn parse_inputs(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut result: Vec<i32> = Vec::new();
    for line in read_file_string(file_path)?.lines() {
        let parsed = line.parse::<i32>()?;
        result.push(parsed);
    }
    Ok(result)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut state: Option<(Option<i32>, i32)> = None;
    let mut count = 0;

    for parsed in parse_inputs("days/2021/day1.txt")? {
        match state {
            Some(tup) => match tup {
                (Some(prev), cur) => {
                    if prev < cur {
                        count += 1;
                    }
                    state = Some((Some(cur), parsed))
                }
                (None, cur) => state = Some((Some(cur), parsed)),
            },
            None => state = Some((None, parsed)),
        }
    }
    Ok(count)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let inputs = parse_inputs("days/2021/day1.txt")?;
    let mut prev: Option<i32> = None;
    let mut count = 0;

    for sum in inputs
        .windows(3)
        .map(|window| window.into_iter().sum::<i32>())
    {
        match prev {
            Some(p) => {
                if p < sum {
                    count += 1;
                }
                prev = Some(sum);
            }
            None => prev = Some(sum),
        }
    }
    Ok(count)
}
