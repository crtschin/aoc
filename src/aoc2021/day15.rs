use std::{error::Error, vec};

use pathfinding::prelude::*;
use typenum::{Unsigned, U1, U5};

use crate::aoc2021::util::read_file;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn neighbors<I: Unsigned>(&self, map: &Map) -> Vec<(Point, u16)> {
        let mut result = Vec::with_capacity(4);
        let mut p;
        if self.x > 0 {
            p = Point {
                x: self.x - 1,
                y: self.y,
            };
            let score = p.score(map);
            result.push((p, score));
        }
        if self.y > 0 {
            p = Point {
                x: self.x,
                y: self.y - 1,
            };
            let score = p.score(map);
            result.push((p, score));
        }
        if self.x < map.width * I::to_u16() - 1 {
            p = Point {
                x: self.x + 1,
                y: self.y,
            };
            let score = p.score(map);
            result.push((p, score));
        }
        if self.y < map.height * I::to_u16() - 1 {
            p = Point {
                x: self.x,
                y: self.y + 1,
            };
            let score = p.score(map);
            result.push((p, score));
        }
        result
    }

    fn score(&self, map: &Map) -> u16 {
        let f = self.x / map.width + self.y / map.height;
        let mut s = map.nodes[(self.y % map.height) as usize][(self.x % map.width) as usize] + f;
        if s > 9 {
            s -= 9;
        }
        s
    }
}

struct Map {
    height: u16,
    width: u16,
    nodes: Vec<Vec<u16>>,
}

fn parse_inputs(file_path: &str) -> Result<Map, Box<dyn Error>> {
    let mut nodes = vec![];
    let mut height = 0;
    let mut width = 0;
    for (y, line) in read_file(file_path)?.lines().enumerate() {
        width = line.len();
        nodes.push(vec![]);
        for c in line.trim_end().bytes() {
            nodes[y].push((c - b'0') as u16);
        }
        height = y + 1;
    }

    Ok(Map {
        height: height as u16,
        width: width as u16,
        nodes,
    })
}

fn solution<I: Unsigned>() -> Result<u16, Box<(dyn std::error::Error + 'static)>> {
    let map = parse_inputs("days/2021/day15.txt")?;
    let w = map.width as u16;
    let h = map.height as u16;
    let goal = Point {
        x: w * I::to_u16() - 1,
        y: h * I::to_u16() - 1,
    };
    const START: Point = Point { x: 0, y: 0 };
    let result = dijkstra(
        &START,
        |p| p.neighbors::<I>(&map),
        |p| *p == goal,
    )
    .unwrap();
    Ok(result.1)
}

pub fn first() -> Result<u16, Box<(dyn std::error::Error + 'static)>> {
    solution::<U1>()
}
pub fn second() -> Result<u16, Box<(dyn std::error::Error + 'static)>> {
    solution::<U5>()
}
