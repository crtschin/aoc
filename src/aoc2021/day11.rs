use std::error::Error;

use crate::util::read_file_string;

fn parse_inputs(file_path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in read_file_string(file_path)?.lines() {
        result.push(line.trim().bytes().collect());
    }
    Ok(result)
}

fn index(x: usize, y: usize) -> usize {
    x * 10 + y
}

fn check_add(
    stack: &mut Vec<(usize, usize)>,
    board: &mut [u8; 100],
    x: usize,
    y: usize,
) -> () {
    let key = index(x, y);
    board[key] += 1;
    if board[key] >= 9 {
        stack.push((x, y))
    }
}

const SIZE: usize = 10;

fn solution(second: bool) -> Result<u32, Box<dyn Error>> {
    let parsed = parse_inputs("days/2021/day11.txt")?;
    let mut sum = 0;
    let board = &mut [0; SIZE * SIZE];
    let number_of_days = 100;
    let mut i = 0;
    for line in parsed {
        for u in line {
            board[i] = u - b'0';
            i += 1;
        }
    }

    let mut d = 0;
    let mut seen = [false; SIZE * SIZE];
    loop {
        let stack = &mut Vec::new();
        for i in 0..board.len() {
            if board[i] >= 9 {
                let x = i / 10;
                let y = i % 10;
                stack.push((x, y));
            }
        }
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            let key = index(x, y);
            if seen[key] {
                continue;
            }
            seen[key] = true;

            if x > 0 {
                if y > 0 {
                    check_add(stack, board, x - 1, y - 1);
                }
                check_add(stack, board, x - 1, y);
                if y < SIZE - 1 {
                    check_add(stack, board, x - 1, y + 1);
                }
            }
            if x < SIZE - 1 {
                if y > 0 {
                    check_add(stack, board, x + 1, y - 1);
                }
                check_add(stack, board, x + 1, y);
                if y < SIZE - 1 {
                    check_add(stack, board, x + 1, y + 1);
                }
            }
            if y > 0 {
                check_add(stack, board, x, y - 1);
            }
            if y < SIZE - 1 {
                check_add(stack, board, x, y + 1);
            }
        }

        let mut count = 0;
        for i in 0..board.len() {
            let o = board[i];
            if o >= 9 {
                count += 1;
                board[i] = 0;
            } else {
                board[i] += 1;
            }
            seen[i] = false;
        }
        if second {
            if count == (SIZE * SIZE) as u32 {
                return Ok(d + 1);
            }
        } else {
            sum += count;
            if d == number_of_days - 1 {
                return Ok(sum);
            }
        }
        d += 1
    }
}

pub fn first() -> Result<u32, Box<dyn Error>> {
    solution(false)
}

pub fn second() -> Result<u32, Box<dyn Error>> {
    solution(true)
}
