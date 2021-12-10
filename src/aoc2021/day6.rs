use std::error::Error;
use std::vec;

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

fn solution(number_of_days: usize) -> Result<i64, Box<dyn Error>> {
    let inputs = parse_inputs("days/2021/day6.txt")?;
    let mut fishes: Vec<i64> = vec![0; 9];
    for i in inputs {
        fishes[i] += 1;
    }
    let memo = &mut vec![0; 8 * number_of_days + 9];

    fn per_day(mut memo: &mut Vec<i64>, countdown: usize, days: usize) -> i64 {
        let key: usize = days * 8 + countdown;
        if memo[key] != 0 {
            return memo[key];
        }

        if days == 0 {
            return 1;
        } else {
            if countdown == 0 {
                memo[key] = per_day(&mut memo, 8, days - 1) + per_day(&mut memo, 6, days - 1);
            } else {
                memo[key] = per_day(&mut memo, countdown - 1, days - 1);
            }
        }
        return memo[key];
    }
    let mut sum = 0;
    for (countdown, &fish) in fishes.iter().enumerate() {
        sum += fish * per_day(memo, countdown, number_of_days);
    }
    Ok(sum)
}

pub fn first() -> Result<i64, Box<dyn Error>> {
    solution(80)
}

pub fn second() -> Result<i64, Box<dyn Error>> {
    solution(256)
}
