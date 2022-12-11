use std::error::Error;
use crate::util::{read_file, to_u8};

struct Instr {
    from: u8,
    to: u8,
    amount: u8,
}

struct Col {
    boxes: Vec<char>,
}

fn parse_input() -> (Vec<Col>, Vec<Instr>) {
    let mut instr_flag = false;
    let mut number_of_columns = 0;
    let mut boxes = Vec::new();
    let mut instrs = Vec::new();
    for line in read_file("days/2022/day5.txt").unwrap().split(|&c| c == b'\n') {
        let length = line.len();
        if number_of_columns == 0 {
            number_of_columns = (length / 4) + 1;
            for _ in 0..number_of_columns {
                boxes.push(Col { boxes: Vec::new() });
            }
        }

        if length == 0 {
            if !instr_flag {
                instr_flag = true;
                for box_ in boxes.iter_mut() {
                    box_.boxes.reverse()
                }
            }
            continue;
        }

        if !instr_flag {
            if line.contains(&b'[') {
                for (i, chunk) in line.chunks(4).enumerate() {
                    let c = chunk[1] as char;
                    if c == ' ' { continue }
                    boxes[i].boxes.push(c);
                }
            } else {
                continue;
            }
        } else {
            let split = line.split(|&c| c == b' ').collect::<Vec<&[u8]>>();
            instrs.push(Instr {
                from: to_u8(split[3]) - 1,
                to: to_u8(split[5]) - 1,
                amount: to_u8(split[1]),
            });
        }
    }
    (boxes, instrs)
}

fn simulate<'a>(boxes: &'a mut Vec<Col>, instrs: &Vec<Instr>, reverse: bool) -> &'a mut Vec<Col> {
    for instr in instrs {
        let length = boxes[instr.from as usize].boxes.len();
        let moving = boxes[instr.from as usize].boxes.split_off(length - instr.amount as usize);
        if reverse {
            boxes[instr.to as usize].boxes.extend(moving.iter().rev());
        } else {
            boxes[instr.to as usize].boxes.extend(moving);
        }
    }
    boxes
}

pub fn first() -> Result<String, Box<dyn Error>> {
    let (mut boxes, instrs) = parse_input();
    simulate(&mut boxes, &instrs, true);
    let result = boxes.iter().map(|box_| *box_.boxes.last().unwrap()).collect::<String>();
    Ok(result)
}

pub fn second() -> Result<String, Box<dyn Error>> {
    let (mut boxes, instrs) = parse_input();
    simulate(&mut boxes, &instrs, false);
    let result = boxes.iter().map(|box_| *box_.boxes.last().unwrap()).collect::<String>();
    Ok(result)
}
