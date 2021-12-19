#![cfg_attr(test, feature(test))]

use std::collections::HashSet;
use std::ops::{Add, Sub};

use uuid::Uuid;

type Input = Vec<Scanner>;
type Output = usize;

fn parse(s: &str) -> Input {
    let mut lines = s.lines().map(|l| l.trim()).peekable();
    let mut scanners = vec![];
    while lines.peek().is_some() {
        let mut scanner = Scanner::new();
        lines.next(); // skip scanner header
        for line in lines.by_ref().take_while(|l| !l.is_empty()) {
            let mut xyz = line.split(",").map(|v| v.parse::<i32>().unwrap());
            scanner.beacons.push(Beacon {
                x: xyz.next().unwrap(),
                y: xyz.next().unwrap(),
                z: xyz.next().unwrap(),
            });
        }
        scanners.push(scanner);
    }
    scanners
}

#[derive(Copy, Clone)]
struct Vec3(i32, i32, i32);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

// https://i.imgur.com/oXVjsTR.png
// xyz = rgb
// right-hand rule

impl Beacon {
    fn rotate_x(mut self, n: u8) -> Self {
        for _ in 0..n {
            self = Self {
                x: self.x,
                y: -self.z,
                z: self.y,
            }
        }
        self
    }

    fn rotate_y(mut self, n: u8) -> Self {
        for _ in 0..n {
            self = Self {
                x: self.z,
                y: self.y,
                z: -self.x,
            }
        }
        self
    }

    fn rotate_z(mut self, n: u8) -> Self {
        for _ in 0..n {
            self = Self {
                x: -self.y,
                y: self.x,
                z: self.z,
            }
        }
        self
    }

    fn rotated(mut self, rotations: &[Rotation]) -> Self {
        for rot in rotations {
            let f = match rot.axis {
                Axis::X => Self::rotate_x,
                Axis::Y => Self::rotate_y,
                Axis::Z => Self::rotate_z,
            };
            self = f(self, rot.amount);
        }
        self
    }

    fn pos(&self) -> Vec3 {
        Vec3(self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Beacon {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    axis: Axis,
    amount: u8,
}

impl Rotation {
    const fn x(amount: u8) -> Self {
        Self {
            axis: Axis::X,
            amount,
        }
    }

    const fn y(amount: u8) -> Self {
        Self {
            axis: Axis::Y,
            amount,
        }
    }

    const fn z(amount: u8) -> Self {
        Self {
            axis: Axis::Z,
            amount,
        }
    }

    const fn all() -> [[Self; 2]; 24] {
        let (x, y, z) = (Self::x, Self::y, Self::z);
        [
            [x(0), x(0)],
            [x(1), x(0)],
            [x(2), x(0)],
            [x(3), x(0)],
            // ...
            [x(0), y(1)],
            [x(1), y(1)],
            [x(2), y(1)],
            [x(3), y(1)],
            // ...
            [x(0), y(2)],
            [x(1), y(2)],
            [x(2), y(2)],
            [x(3), y(2)],
            // ...
            [x(0), y(3)],
            [x(1), y(3)],
            [x(2), y(3)],
            [x(3), y(3)],
            // ...
            [x(0), z(1)],
            [x(1), z(1)],
            [x(2), z(1)],
            [x(3), z(1)],
            // ...
            [x(0), z(3)],
            [x(1), z(3)],
            [x(2), z(3)],
            [x(3), z(3)],
        ]
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
    id: Uuid,
}

impl Scanner {
    fn new() -> Self {
        Self {
            beacons: vec![],
            id: Uuid::new_v4(),
        }
    }

    fn rotated(&self, rotations: &[Rotation]) -> Self {
        Self {
            beacons: self.beacons.iter().map(|b| b.rotated(rotations)).collect(),
            id: self.id,
        }
    }

    fn all_rotations(&self) -> [Self; 24] {
        Rotation::all().map(|r| self.rotated(&r))
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.beacons
            .iter()
            .filter(|b| other.beacons.contains(b))
            .count()
            >= 12
    }
}

impl Add<Vec3> for Scanner {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            beacons: self.beacons.into_iter().map(|b| b + rhs).collect(),
            id: self.id,
        }
    }
}

fn unify(inp: &Input) -> (Vec<Scanner>, Vec<Vec3>) {
    let mut region = vec![inp[0].clone()];

    let mut scanners = inp[1..].to_vec();
    let mut offsets = vec![];
    let mut failures = HashSet::new();
    while !scanners.is_empty() {
        let scanner = scanners.remove(0);
        let mut matching_scanner = None;

        'found: for test_scanner in &region {
            if failures.contains(&(test_scanner.id, scanner.id)) {
                continue;
            }

            for rotated in scanner.all_rotations() {
                for test_beacon in &test_scanner.beacons {
                    for comparison_beacon in &rotated.beacons {
                        let delta = test_beacon.pos() - comparison_beacon.pos();
                        let translated = rotated.clone() + delta;
                        if test_scanner.overlaps(&translated) {
                            matching_scanner = Some((translated, delta));
                            break 'found;
                        }
                    }
                }
            }

            failures.insert((test_scanner.id, scanner.id));
        }

        if let Some((s, delta)) = matching_scanner {
            region.push(s);
            offsets.push(delta);
        } else {
            // better luck next time
            scanners.push(scanner);
        }
    }

    (region, offsets)
}

fn part1(inp: &Input) -> Output {
    let (region, _) = unify(inp);

    region
        .into_iter()
        .flat_map(|s| s.beacons)
        .collect::<HashSet<Beacon>>()
        .len()
}

fn part2(inp: &Input) -> Output {
    let (_, offsets) = unify(inp);

    let mut max_dist = 0;
    for a in &offsets {
        for b in &offsets {
            let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
            max_dist = max_dist.max(dist);
        }
    }
    max_dist as _
}

util::register_alt!(parse, part1, part2);
