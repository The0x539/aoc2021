#![cfg_attr(test, feature(test))]

use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

struct Movement {
    direction: Direction,
    amount: i32,
}

impl FromStr for Movement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.find(' ').ok_or(())?;
        let (l, r) = s.split_at(i);
        let direction = match l {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => return Err(()),
        };
        let amount = r[1..].parse().map_err(drop)?;
        Ok(Self { direction, amount })
    }
}

fn part1(input: &[Movement]) -> usize {
    let mut x = 0;
    let mut y = 0;
    for m in input {
        match m.direction {
            Direction::Forward => x += m.amount,
            Direction::Down => y += m.amount,
            Direction::Up => y -= m.amount,
        }
    }
    (x * y) as usize
}

fn part2(input: &[Movement]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for m in input {
        match m.direction {
            Direction::Down => aim += m.amount,
            Direction::Up => aim -= m.amount,
            Direction::Forward => {
                x += m.amount;
                y += m.amount * aim;
            }
        }
    }
    (x * y) as usize
}

util::register!(|l| Movement::from_str(l).unwrap(), part1, part2);
