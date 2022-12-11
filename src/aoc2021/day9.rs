use std::error::Error;

use crate::util::read_file_string;

fn parse_inputs(file_path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in read_file_string(file_path)?.lines() {
        result.push(line.trim().chars().collect());
    }
    Ok(result)
}

fn lowest(parsed: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    for x in 0..parsed.len() {
        for y in 0..parsed[x].len() {
            let mut check = true;
            let cur = parsed[x][y];
            if x > 0 {
                check = check && parsed[x - 1][y] > cur;
            }
            if x < parsed.len() - 1 {
                check = check && parsed[x + 1][y] > cur;
            }
            if y > 0 {
                check = check && parsed[x][y - 1] > cur;
            }
            if y < parsed[x].len() - 1 {
                check = check && parsed[x][y + 1] > cur;
            }
            if check {
                results.push((x, y));
            }
        }
    }
    results
}

pub fn first() -> Result<u16, Box<dyn Error>> {
    let parsed = parse_inputs("days/2021/day9.txt")?;
    let mut sum = 0;
    for (x, y) in lowest(&parsed) {
        sum += parsed[x][y] as u16 - '0' as u16 + 1;
    }
    Ok(sum)
}

pub fn second() -> Result<u8, Box<dyn Error>> {
    let parsed = parse_inputs("days/2021/day9.txt")?;
    let mut stack: Vec<_> = lowest(&parsed)
        .iter()
        .enumerate()
        .map(|(x, &y)| (x, y))
        .collect();
    let size = parsed.len();
    let mut seen = vec![false; size * size];
    let mut basins = vec![0; stack.len()];
    let n = basins.len();
    while stack.len() > 0 {
        let (basin_index, (x, y)) = stack.pop().unwrap();
        let key = x * size + y;
        if parsed[x][y] == '9' || seen[key] {
            continue;
        }

        basins[basin_index] += 1;
        seen[key] = true;
        if x > 0 {
            stack.push((basin_index, (x - 1, y)));
        }
        if x < parsed.len() - 1 {
            stack.push((basin_index, (x + 1, y)));
        }
        if y > 0 {
            stack.push((basin_index, (x, y - 1)));
        }
        if y < parsed[0].len() - 1 {
            stack.push((basin_index, (x, y + 1)));
        }
    }
    basins.select_nth_unstable(n - 3);
    Ok(basins[n - 3..n].iter().product())
}
