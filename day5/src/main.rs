use either::{Left, Right};
use std::str::FromStr;

struct Vec2 {
    x: usize,
    y: usize,
}

struct Line(Vec2, Vec2);

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(" -> ").ok_or(())?;
        let (x1, y1) = l.split_once(",").ok_or(())?;
        let (x2, y2) = r.split_once(",").ok_or(())?;
        Ok(Line(
            Vec2 {
                x: x1.parse().map_err(drop)?,
                y: y1.parse().map_err(drop)?,
            },
            Vec2 {
                x: x2.parse().map_err(drop)?,
                y: y2.parse().map_err(drop)?,
            },
        ))
    }
}

fn solution(input: &[Line], diagonals: bool) -> usize {
    let x_max = input
        .iter()
        .flat_map(|l| [l.0.x, l.1.x])
        .max()
        .unwrap_or_default()
        + 1;
    let y_max = input
        .iter()
        .flat_map(|l| [l.0.y, l.1.y])
        .max()
        .unwrap_or_default()
        + 1;

    let range = |v1: usize, v2: usize| {
        if v1 < v2 {
            Left(v1..=v2)
        } else {
            Right((v2..=v1).rev())
        }
    };

    let mut grid = vec![vec![0; x_max]; y_max];
    for line in input {
        if line.0.x == line.1.x {
            let x = line.0.x;
            for y in range(line.0.y, line.1.y) {
                grid[y][x] += 1;
            }
        } else if line.0.y == line.1.y {
            let y = line.0.y;
            for x in range(line.0.x, line.1.x) {
                grid[y][x] += 1;
            }
        } else if diagonals {
            let ys = range(line.0.y, line.1.y);
            let xs = range(line.0.x, line.1.x);
            for (y, x) in ys.zip(xs) {
                grid[y][x] += 1;
            }
        }
    }

    grid.iter().flatten().filter(|n| **n >= 2).count()
}

fn part1(input: &[Line]) -> usize {
    solution(input, false)
}

fn part2(input: &[Line]) -> usize {
    solution(input, true)
}

util::register!(util::parse_unwrap::<Line>, part1, part2);
