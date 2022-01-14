use std::{collections::HashSet, error::Error, vec};

use ahash::RandomState;

use crate::aoc2021::util::read_file;

struct State {
    dots: HashSet<(u16, u16), RandomState>,
    width: u16,
    height: u16,
}
enum Instr {
    Horiz(u16),
    Vert(u16),
}

fn parse_inputs(file_path: &str) -> Result<(State, Vec<Instr>), Box<dyn Error>> {
    let mut flag = false;
    let mut dots = HashSet::default();
    let mut instrs = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in read_file(file_path)?.lines() {
        if line.len() == 0 {
            flag = true;
            continue;
        }
        if flag {
            let instr = line.split(" ").nth(2).unwrap();
            let mut parsed = instr.split("=");
            match parsed.next().unwrap() {
                "x" => instrs.push(Instr::Horiz(parsed.next().unwrap().parse::<u16>().unwrap())),
                "y" => instrs.push(Instr::Vert(parsed.next().unwrap().parse::<u16>().unwrap())),
                _ => unreachable!(),
            }
        } else {
            let mut point = line.split(",");
            let x = point.next().unwrap().parse::<u16>().unwrap();
            let y = point.next().unwrap().parse::<u16>().unwrap();
            width = width.max(x);
            height = height.max(y);
            dots.insert((x, y));
        }
    }
    Ok((
        State {
            dots,
            width,
            height,
        },
        instrs,
    ))
}

fn _debug_state(state: &State) -> () {
    let mut result = String::default();
    let mut state_ = vec![vec!['.'; state.width as usize + 1]; state.height as usize];
    for (x, y) in state.dots.iter() {
        state_[*y as usize][*x as usize] = '#';
    }
    for line in state_ {
        result.push_str(&line.iter().collect::<String>());
        result.push('\n');
    }
    print!("{}", result);

}

fn solution(number_of_folds: usize, should_print: bool) -> Result<usize, Box<dyn Error>> {
    let (mut state, instrs) = parse_inputs("days/2021/day13.txt")?;
    let n = if number_of_folds == 0 {
        instrs.len()
    } else {
        number_of_folds
    };

    for instr in instrs.iter() {
        match instr {
            Instr::Horiz(i) => state.width = state.width.min(*i),
            Instr::Vert(i) => state.height = state.height.min(*i),
        }
    }

    let mut new_state = HashSet::default();
    for (x, y) in state.dots {
        let mut x_ = x;
        let mut y_ = y;
        for instr in instrs.iter().take(n) {
            match instr {
                Instr::Vert(i) => {
                    if y_ > *i {
                        y_ = 2 * i - y_;
                    }
                }
                Instr::Horiz(i) => {
                    if x_ > *i {
                        x_ = 2 * i - x_;
                    }
                }
            }
        }
        new_state.insert((x_, y_));
    }
    state.dots = new_state;
    if should_print {
        _debug_state(&state);
    }
    Ok(state.dots.len())
}

pub fn first() -> Result<usize, Box<dyn Error>> {
    solution(1, false)
}

pub fn second(should_print: bool) -> Result<usize, Box<dyn Error>> {
    solution(0, should_print)
}
