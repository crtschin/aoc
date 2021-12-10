use std::collections::{HashMap, HashSet};
use std::error::Error;

use ahash::RandomState;

use super::util::read_file;

struct Board {
    numbers: Vec<i32>,
}

fn parse_inputs(
    file_path: &str,
) -> Result<(Vec<i32>, Vec<Board>), Box<(dyn std::error::Error + 'static)>> {
    let raw = read_file(file_path)?;
    let mut lines = raw.lines();
    let mut result: Vec<_> = Vec::new();
    let draws = lines.next().unwrap()
        .trim_end()
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut board: Vec<i32> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        board.append(
            &mut line
                .trim_end()
                .split(" ")
                .filter_map(|raw| match raw {
                    "" => None,
                    i => Some(i.parse::<i32>().unwrap()),
                })
                .collect::<Vec<i32>>(),
        );

        if board.len() == 25 {
            result.push(Board { numbers: board });
            board = Vec::new();
        }
    }
    Ok((draws, result))
}

type BoardIdx = Box<Vec<(usize, usize)>>;
type BoardSet = Vec<HashSet<i32, RandomState>>;

fn solution(last: bool) -> Result<i32, Box<dyn Error>> {
    let (draws, board) = parse_inputs("days/2021/day4.txt")?;
    let mut registry: HashMap<i32, BoardIdx, RandomState> = HashMap::default();
    let mut entries: Vec<BoardSet> = Vec::new();
    for (b_idx, board) in board.iter().enumerate() {
        let mut entry = vec![HashSet::default(); 10];
        for (i, row) in board.numbers.chunks(5).enumerate() {
            for (j, number) in row.iter().enumerate() {
                // Insert row entries
                entry[i].insert(*number);
                if !registry.contains_key(number) {
                    registry.insert(*number, Box::new(vec![(b_idx, i)]));
                } else {
                    registry.get_mut(&number).unwrap().push((b_idx, i));
                }
                // Insert col entries
                let col_idx = 5 + j % 5;
                entry[col_idx].insert(*number);
                registry.get_mut(&number).unwrap().push((b_idx, col_idx));
            }
        }
        entries.push(entry);
    }

    let mut won: HashSet<_, RandomState> = HashSet::default();
    for draw in draws {
        match registry.get(&draw) {
            Some(index) => {
                for (bi, i) in index.iter() {
                    entries[*bi][*i].remove(&draw);
                    if entries[*bi][*i].len() == 0  {
                        if won.contains(bi) {
                            continue;
                        } else {
                            won.insert(bi);
                        }
                        if last {
                            if entries.len() != won.len() {
                                continue;
                            }
                        }
                        let mut sum: i32 = 0;
                        for row in 0..5 {
                            sum += entries[*bi][row].iter().sum::<i32>();
                        }
                        return Ok(sum * draw);
                    }
                }
            }
            None => {}
        }
    }
    Ok(-1)
}

pub fn first() -> Result<i32, Box<dyn Error>> {
    solution(false)
}

pub fn second() -> Result<i32, Box<dyn Error>> {
    solution(true)
}
