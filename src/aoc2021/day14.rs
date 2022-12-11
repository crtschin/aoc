use std::{collections::HashMap, error::Error};

use ahash::RandomState;

use crate::util::read_file_string;

type Template = Vec<u8>;
type Rules = HashMap<(u8, u8), u8, RandomState>;

fn parse_inputs(file_path: &str) -> Result<(Template, Rules), Box<dyn Error>> {
    let mut flag = true;
    let mut template = vec![];
    let mut rules = HashMap::default();
    for line in read_file_string(file_path)?.lines() {
        if line.len() == 0 {
            flag = false;
            continue;
        }

        if flag {
            template = line.bytes().collect();
        } else {
            let mut split = line.split(" -> ");
            let from = split.next().unwrap().as_bytes();
            let to = split.next().unwrap().as_bytes();
            rules.insert((from[0], from[1]), to[0]);
        }
    }
    Ok((template, rules))
}

fn solution(steps: usize) -> Result<u64, Box<dyn Error>> {
    let (mut template, rules) = parse_inputs("days/2021/day14.txt")?;
    let mut state: HashMap<(u8, u8), u64, RandomState> = HashMap::default();
    let mut template_ = vec![b'0'];
    template_.append(&mut template);
    template_.push(b'0');
    for pair in template_.windows(2) {
        let entry = state.entry((pair[0], pair[1])).or_insert(0);
        *entry += 1;
    }

    for _ in 0..steps {
        let mut state_: HashMap<(u8, u8), u64, RandomState> = HashMap::default();
        for (pair, count) in state.iter() {
            match rules.get(pair) {
                Some(c) => {
                    let mut e = state_.entry(((*pair).0, *c)).or_insert(0);
                    *e += *count;

                    e = state_.entry((*c, (*pair).1)).or_insert(0);
                    *e += *count;
                }
                None => {
                    let e = state_.entry(*pair).or_insert(0);
                    *e += *count;
                }
            };
        }
        state = state_;
    }

    let mut counts = [0; 26];
    for ((c, _), count) in state.iter() {
        if *c == b'0' { continue; }
        counts[*c as usize - 'A' as usize] += count;
    }

    let mut min = u64::MAX;
    let mut max = 0;
    for v in counts.iter() {
        if *v != 0 {
            min = min.min(*v);
        }
        max = max.max(*v);
    }
    Ok(max - min)
}

pub fn first() -> Result<u64, Box<dyn Error>> {
    solution(10)
}
pub fn second() -> Result<u64, Box<dyn Error>> {
    solution(40)
}
