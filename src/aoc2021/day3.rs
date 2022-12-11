use std::error::Error;

use crate::util::read_file_string;

fn parse_inputs(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in read_file_string(file_path)?.lines() {
        result.push(String::from(line.trim_end()));
    }
    Ok(result)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    const NUMBER_DIGITS: usize = 12;
    let mut state: [i32; NUMBER_DIGITS] = [0; NUMBER_DIGITS];
    for input in parse_inputs("days/2021/day3.txt")? {
        for (i, b) in input.chars().enumerate() {
            if b == '1' {
                state[i] += 1;
            } else {
                state[i] -= 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for digit in state {
        if digit > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    Ok(i32::from_str_radix(gamma.as_str(), 2).unwrap()
        * i32::from_str_radix(epsilon.as_str(), 2).unwrap())
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    const NUMBER_DIGITS: usize = 12;
    let integers = parse_inputs("days/2021/day3.txt")?;
    let mut oxygen_filtered: Vec<_> = integers.clone();
    let mut co2_filtered: Vec<_> = integers;

    let per_condition = |i, inputs: Vec<String>, cond: fn(usize, usize) -> bool| -> Vec<String> {
        let mut temp = (Vec::new(), Vec::new());
        for number in inputs {
            if number.as_bytes()[i] == b'1' {
                temp.1.push(number);
            } else {
                temp.0.push(number);
            }
        }

        if cond(temp.0.len(), temp.1.len()) {
            return temp.0;
        } else {
            return temp.1;
        }
    };

    for i in 0..NUMBER_DIGITS {
        if oxygen_filtered.len() > 1 {
            oxygen_filtered = per_condition(i, oxygen_filtered, |t1, t2| t1 > t2);
        }
        if co2_filtered.len() > 1 {
            co2_filtered = per_condition(i, co2_filtered, |t1, t2| t1 <= t2);
        }
    }

    Ok(i32::from_str_radix(oxygen_filtered[0].as_str(), 2).unwrap()
        * i32::from_str_radix(co2_filtered[0].as_str(), 2).unwrap())
}
