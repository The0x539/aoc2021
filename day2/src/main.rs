#![feature(array_windows)]

use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Down,
    Up,
}

struct Movement {
    direction: Direction,
    amount: i32,
}

impl Movement {
    fn from_str(s: &str) -> Option<Self> {
        let i = s.find(' ')?;
        let (l, r) = s.split_at(i);
        let direction = match l {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => return None,
        };
        let amount = r[1..].parse().ok()?;
        Some(Self { direction, amount })
    }
}

fn parse(input: &str) -> Vec<Movement> {
    BufReader::new(File::open(input).unwrap())
        .lines()
        .map(|s| Movement::from_str(s.unwrap().trim()).unwrap())
        .collect()
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

fn main() {
    let input = parse("input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
fn parse_out(output: &str) -> (usize, usize) {
    let f = BufReader::new(File::open(output).unwrap());
    let mut l = f.lines();
    let mut n = || l.next().unwrap().unwrap().trim().parse().unwrap();
    (n(), n())
}

#[cfg(test)]
#[test]
fn test() {
    let input = parse("test.txt");
    let (a, b) = parse_out("test.out.txt");
    assert_eq!(part1(&input), a);
    assert_eq!(part2(&input), b);
}
