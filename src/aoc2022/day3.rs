use std::error::Error;

use crate::util::read_file;

fn score_letter(c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        return c - b'A' + 27;
    } else {
        return c - b'a' + 1;
    }
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    for line in read_file("days/2022/day3.txt")?.split(|&c| c == b'\n') {
        let comps = line.split_at(line.len() / 2);
        for &ch in comps.0 {
            match comps.1.iter().find(|&&c| c == ch) {
                Some(&s) => {
                    score += score_letter(s) as i32;
                    break;
                }
                None => (),
            };
        }
    }
    Ok(score)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    let mut score = 0;
    let mut i = 0;
    let mut tracker: Vec<u8> = vec![];
    for line in read_file("days/2022/day3.txt")?.split(|&c| c == b'\n') {
        if i == 0 {
            tracker = line.to_vec();
        } else {
            tracker.retain(|&c| line.contains(&c));
        }

        if i == 2 {
            score += score_letter(tracker[0]) as i32;
        }

        i = (i + 1) % 3;
    }
    Ok(score)
}
