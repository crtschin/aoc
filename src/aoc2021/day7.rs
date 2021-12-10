use std::collections::HashSet;
use std::error::Error;
use std::ops::Div;

use ahash::RandomState;

use super::util::read_file;

fn parse_inputs(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in read_file(file_path)?.lines() {
        for i in line
            .trim_end()
            .split(",")
            .map(|n| n.parse::<_>().unwrap())
        {
            result.push(i);
        }
    }
    Ok(result)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    // corresponds to finding the median of the array
    // https://en.wikipedia.org/wiki/Geometric_median
    let inputs = &mut parse_inputs("days/2021/day7.txt")?;
    let n = inputs.len();
    let compare = |a: &i32, b: &i32| a.cmp(b);
    let median: i32;
    if n.rem_euclid(2) == 1 {
        let (_, median_, _) = inputs.select_nth_unstable_by(n.div(2), compare);
        median = *median_;
    } else {
        let (search, high, _) = inputs.select_nth_unstable_by(n.div(2), compare);
        let (_, low, _) = search.select_nth_unstable_by(search.len() - 1, compare);
        median = (*high + *low) / 2;
    }
    Ok(inputs.iter().map(|i| (i - median).abs()).sum::<i32>())
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let inputs = &parse_inputs("days/2021/day7.txt")?;
    let mut fuel = i32::MAX;
    fn do_sum(to: i32) -> i32 { to * (to + 1) / 2 }
    let mut seen: HashSet<_, RandomState> = HashSet::default();
    for i in inputs {
        if !seen.contains(&i) {
            fuel = fuel.min(inputs.iter().map(|&j| do_sum((i - j).abs())).sum());
            seen.insert(i);
        }
    }
    Ok(fuel)
}
