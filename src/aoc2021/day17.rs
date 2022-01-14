use std::error::Error;

#[derive(Debug)]
struct Target {
    x: (i16, i16),
    y: (i16, i16),
}

fn parse_inputs() -> Result<Target, Box<dyn Error>> {
    let mut raw = include_str!("../../days/2021/day17.txt").trim()[13..].split(", ");
    let mut x = (raw.next().unwrap()[2..]).split("..");
    let mut y = (raw.next().unwrap()[2..]).split("..");
    Ok(Target {
        x: (
            x.next().unwrap().parse().unwrap(),
            x.next().unwrap().parse().unwrap(),
        ),
        y: (
            y.next().unwrap().parse().unwrap(),
            y.next().unwrap().parse().unwrap(),
        ),
    })
}

pub fn first() -> Result<i16, Box<dyn Error>> {
    let target = parse_inputs()?;
    const MV: i16 = 128;
    let mut ty = target.y.0;
    let mut tvy = 0;
    let mut vy = 0;
    let difference = target.y.1 - target.y.0;
    let precalculate = (0..MV).map(|i| i * (i + 1) / 2).collect::<Vec<_>>();
    for _ in 0..MV {
        tvy += 1;
        ty += tvy;
        if ty < 0 {
            continue;
        }
        for (vy_, y) in precalculate.iter().enumerate().rev() {
            if *y >= ty && *y <= ty + difference {
                vy = vy_ as i16;
                break;
            }
        }
    }

    Ok(vy * (vy + 1) / 2)
}

fn lands_in(mut velocity: (i16, i16), target: &Target) -> bool {
    let mut x = 0;
    let mut y = 0;
    loop {
        x += velocity.0;
        y += velocity.1;
        if x >= target.x.0 && y <= target.y.1 && x <= target.x.1 && y >= target.y.0 {
            return true;
        } else if (velocity.0 == 0 && x < target.x.0) || x >= target.x.1 || y <= target.y.0 {
            return false;
        }
        velocity = ((velocity.0 - 1).max(0), (velocity.1 - 1));
    }
}

pub fn second() -> Result<i16, Box<dyn Error>> {
    let target = parse_inputs()?;
    let mut count = 0;
    for vx in 0..target.x.1 + 1 {
        for vy in target.y.0..-target.y.0 {
            if lands_in((vx, vy), &target) {
                count += 1;
            }
        }
    }
    Ok(count)
}
