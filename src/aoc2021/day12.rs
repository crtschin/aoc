use std::{collections::HashMap, error::Error};

use ahash::RandomState;

use crate::aoc2021::util::read_file;

struct Cave {
    start: usize,
    end: usize,
    links: Vec<Vec<usize>>,
    small: Vec<bool>,
}

fn parse_inputs(file_path: &str) -> Result<Cave, Box<dyn Error>> {
    let mut links = vec![];
    let mut small = vec![];
    let mut lookup: HashMap<&str, usize, RandomState> = HashMap::default();
    let mut start = 0;
    let mut end = 0;
    for line in read_file(file_path)?.lines() {
        let splits: Vec<_> = line.split("-").collect();
        for dest in splits.iter() {
            if !lookup.contains_key(dest) {
                let index = links.len();
                links.push(Vec::new());
                lookup.insert(dest, index);
                if *dest == dest.to_ascii_lowercase() {
                    small.push(true);
                } else {
                    small.push(false);
                }

                if *dest == "start" {
                    start = index;
                } else if *dest == "end" {
                    end = index;
                }
            }
        }
        let x = lookup[splits[0]];
        let y = lookup[splits[1]];
        links[y].push(x);
        links[x].push(y);
    }
    Ok(Cave {
        links,
        start,
        end,
        small,
    })
}

fn solution(second: bool) -> Result<u32, Box<dyn Error>> {
    let cave = parse_inputs("days/2021/day12.txt")?;
    let mut count = 0;
    let mut path: Vec<(bool, usize)> = vec![];
    let mut seen = vec![0_u8; cave.small.len()];
    let mut stack = vec![(0_usize, cave.start)];

    while stack.len() > 0 {
        let (prev, current_node) = stack.pop().unwrap();
        while path.len() > prev {
            let (_, deleted) = path.pop().unwrap();
            if seen[deleted] > 0 {
                seen[deleted] = seen[deleted] - 1;
            }
        }

        if current_node == cave.end {
            count += 1;
            continue;
        } else {
            if cave.small[current_node] {
                let can_twice = if path.len() == 0 {
                    second
                } else {
                    path[path.len() - 1].0
                };

                if seen[current_node] > 0 {
                    if can_twice {
                        seen[current_node] += 1;
                        path.push((false, current_node));
                    } else {
                        continue;
                    }
                } else {
                    seen[current_node] += 1;
                    path.push((can_twice, current_node));
                }
            }

            for link in cave.links[current_node].iter() {
                if *link != cave.start {
                    stack.push((path.len(), *link));
                }
            }
        }
    }
    Ok(count)
}

pub fn first() -> Result<u32, Box<dyn Error>> {
    solution(false)
}

pub fn second() -> Result<u32, Box<dyn Error>> {
    solution(true)
}
