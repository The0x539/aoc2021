#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

struct Algorithm([bool; 512]);

type Input = (Algorithm, Vec<Vec<bool>>);
type Output = usize;

fn parse(s: &str) -> Input {
    let algorithm_bytes = s
        .lines()
        .map(str::trim)
        .take_while(|s| !s.is_empty())
        .flat_map(str::bytes);

    let algorithm = Algorithm(
        algorithm_bytes
            .map(|b| b == b'#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    let mut rows = s.lines().map(str::trim);
    rows.by_ref().take_while(|s| !s.is_empty()).for_each(drop);

    let image = rows
        .map(|row| row.bytes().map(|b| b == b'#').collect())
        .collect();

    (algorithm, image)
}

struct Image {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
    border: bool,
    pixels: HashSet<(i32, i32)>,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.top..self.bottom {
            for x in self.left..self.right {
                if self.pixels.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn new(rows: &Vec<Vec<bool>>) -> Self {
        let mut this = Image {
            top: 0,
            left: 0,
            bottom: rows.len() as i32,
            right: rows[0].len() as i32,
            border: false,
            pixels: HashSet::new(),
        };

        for y in 0..this.bottom {
            for x in 0..this.right {
                if rows[y as usize][x as usize] {
                    this.pixels.insert((x, y));
                }
            }
        }
        this
    }

    fn contains(&self, (x, y): (i32, i32)) -> bool {
        if x < self.left || x >= self.right || y < self.top || y >= self.bottom {
            self.border
        } else {
            self.pixels.contains(&(x, y))
        }
    }

    fn enhance(&self, algorithm: &Algorithm) -> Self {
        let mut new = Image {
            top: self.top - 2,
            left: self.left - 2,
            bottom: self.bottom + 2,
            right: self.right + 2,
            border: false,
            pixels: HashSet::new(),
        };

        for y in new.top..new.bottom {
            for x in new.left..new.right {
                let coords = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];
                let mut index = 0;
                for coord in coords {
                    index = (index << 1) | self.contains(coord) as usize;
                }
                if algorithm.0[index] {
                    new.pixels.insert((x, y));
                }
            }
        }

        new.border = new.pixels.contains(&(new.top, new.left));
        new.top += 1;
        new.left += 1;
        new.bottom -= 1;
        new.right -= 1;

        new
    }
}

fn part1(inp: &Input) -> Output {
    let (ref algorithm, ref rows) = inp;

    let mut image = Image::new(rows);

    for _ in 0..2 {
        image = image.enhance(algorithm);
    }

    image.pixels.len()
}

fn part2(inp: &Input) -> Output {
    let (ref algorithm, ref rows) = inp;

    let mut image = Image::new(rows);

    for _ in 0..50 {
        image = image.enhance(algorithm);
    }

    image.pixels.len()
}

util::register_alt!(parse, part1, part2);
