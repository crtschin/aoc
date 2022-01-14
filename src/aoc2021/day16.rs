use std::{error::Error, vec};

use nom::{
    bytes::complete::take,
    multi::{count, many0},
};

use crate::aoc2021::util::read_file;

fn parse_inputs(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    for c in read_file(file_path)?.trim().bytes() {
        match c {
            b'0' => result.push_str("0000"),
            b'1' => result.push_str("0001"),
            b'2' => result.push_str("0010"),
            b'3' => result.push_str("0011"),
            b'4' => result.push_str("0100"),
            b'5' => result.push_str("0101"),
            b'6' => result.push_str("0110"),
            b'7' => result.push_str("0111"),
            b'8' => result.push_str("1000"),
            b'9' => result.push_str("1001"),
            b'A' => result.push_str("1010"),
            b'B' => result.push_str("1011"),
            b'C' => result.push_str("1100"),
            b'D' => result.push_str("1101"),
            b'E' => result.push_str("1110"),
            b'F' => result.push_str("1111"),
            _ => unreachable!(),
        }
    }
    Ok(result)
}

#[derive(Debug)]
struct Version(u8);
#[derive(Debug)]
struct Type(u8);
#[derive(Debug)]
enum Packet {
    Lit(Version, u64),
    Op(Version, Type, Vec<Packet>),
}

fn expression_parser(input: &str) -> nom::IResult<&str, Packet> {
    let version = take(3_usize)(input)?;
    let type_ = take(3_usize)(version.0)?;
    match type_.1 {
        "100" => {
            let mut result = vec![];
            let mut literal_input = type_.0;
            loop {
                let tag = take(1_usize)(literal_input)?;
                let part = take(4_usize)(tag.0)?;
                result.push(part.1);
                literal_input = part.0;
                match tag.1 {
                    "0" => {
                        return Ok((
                            literal_input,
                            Packet::Lit(
                                Version(u8::from_str_radix(version.1, 2).unwrap()),
                                result.iter().fold(0, |prev, cur| {
                                    prev << 4 | u64::from_str_radix(cur, 2).unwrap()
                                }),
                            ),
                        ))
                    }
                    _ => continue,
                }
            }
        }
        _ => {
            let length_type = take(1_usize)(type_.0)?;
            let sub_packets;
            match length_type.1 {
                "0" => {
                    let content_length = take(15_u8)(length_type.0)?;
                    let (rest, reserved) = take(
                        usize::from_str_radix(content_length.1, 2).unwrap(),
                    )(content_length.0)?;
                    let result = many0(expression_parser)(reserved).unwrap();
                    sub_packets = (rest, result.1);
                }
                _ => {
                    let nr_sub_packets = take(11_usize)(length_type.0)?;
                    sub_packets = count(
                        expression_parser,
                        usize::from_str_radix(nr_sub_packets.1, 2).unwrap(),
                    )(nr_sub_packets.0)?;
                }
            }
            return Ok((
                sub_packets.0,
                Packet::Op(
                    Version(u8::from_str_radix(version.1, 2).unwrap()),
                    Type(u8::from_str_radix(type_.1, 2).unwrap()),
                    sub_packets.1,
                ),
            ));
        }
    }
}

fn solution() -> Result<Packet, Box<dyn Error>> {
    let binary = parse_inputs("days/2021/day16.txt")?;
    let packet = expression_parser(&binary).unwrap();
    Ok(packet.1)
}

pub fn first() -> Result<u32, Box<dyn Error>> {
    let packet = solution()?;
    fn count_version(packet: &Packet) -> u32 {
        match packet {
            Packet::Lit(Version(v), _) => *v as u32,
            Packet::Op(Version(v), _, sub) => {
                *v as u32 + sub.iter().map(count_version).sum::<u32>()
            }
        }
    }
    Ok(count_version(&packet))
}

pub fn second() -> Result<u64, Box<dyn Error>> {
    let packet = solution()?;
    fn eval(packet: &Packet) -> u64 {
        match packet {
            Packet::Lit(_, i) => *i as u64,
            Packet::Op(_, Type(id), sub) => match id {
                0 => sub.iter().map(eval).sum(),
                1 => sub.iter().map(eval).product(),
                2 => sub.iter().map(eval).min().unwrap(),
                3 => sub.iter().map(eval).max().unwrap(),
                5 => (eval(&sub[0]) > eval(&sub[1])).into(),
                6 => (eval(&sub[0]) < eval(&sub[1])).into(),
                7 => (eval(&sub[0]) == eval(&sub[1])).into(),
                _ => unreachable!(),
            },
        }
    }
    Ok(eval(&packet))
}
