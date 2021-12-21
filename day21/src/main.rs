#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

type Input = (u64, u64);
type Output = u64;

struct DeterministicDie {
    inner: std::iter::Flatten<std::iter::Repeat<std::ops::RangeInclusive<u64>>>,
    roll_count: u64,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            inner: std::iter::repeat(1..=100).flatten(),
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u64 {
        self.roll_count += 1;
        self.inner.next().unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn new(pos: u64) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }

    fn forward(&mut self, amount: u64) {
        self.pos += amount;
        self.pos %= 10;
    }

    fn points(&mut self) {
        self.score += self.pos + 1;
    }

    fn play(&mut self, die: &mut DeterministicDie) {
        self.forward(die.roll());
        self.forward(die.roll());
        self.forward(die.roll());
        self.points();
    }
}

fn parse(s: &str) -> Input {
    let positions = s
        .lines()
        .map(str::trim)
        .map(|l| l[l.len() - 2..].trim().parse().unwrap())
        .collect::<Vec<_>>();

    (positions[0], positions[1])
}

fn part1(inp: &Input) -> Output {
    let mut p0 = Player::new(inp.0);
    let mut p1 = Player::new(inp.1);
    let mut die = DeterministicDie::new();

    loop {
        p0.play(&mut die);
        if p0.score >= 1000 {
            return p1.score * die.roll_count;
        }
        p1.play(&mut die);
        if p1.score >= 1000 {
            return p0.score * die.roll_count;
        }
    }
}

struct Multiverse {
    active_worlds: HashMap<[Player; 2], u64>,
    wins: [u64; 2],
}

impl Multiverse {
    fn new(p0_pos: u64, p1_pos: u64) -> Self {
        let players = [p0_pos, p1_pos].map(Player::new);
        let active_worlds = HashMap::from_iter([(players, 1)]);
        Self {
            active_worlds,
            wins: [0, 0],
        }
    }

    fn step(&mut self, i: usize) {
        let old_worlds = std::mem::take(&mut self.active_worlds);
        for (p, count) in old_worlds {
            for roll in 1..=3 {
                let mut new_p = p;
                new_p[i].forward(roll);
                *self.active_worlds.entry(new_p).or_default() += count;
            }
        }
    }

    fn points(&mut self, i: usize) {
        let old_worlds = std::mem::take(&mut self.active_worlds);
        for (mut p, count) in old_worlds {
            p[i].points();
            if p[i].score >= 21 {
                self.wins[i] += count;
            } else {
                *self.active_worlds.entry(p).or_default() += count;
            }
        }
    }

    fn turn(&mut self, i: usize) {
        self.step(i);
        self.step(i);
        self.step(i);
        self.points(i);
    }

    fn conclude(&self) -> Option<u64> {
        if self.active_worlds.is_empty() {
            Some(self.wins.into_iter().max().unwrap())
        } else {
            None
        }
    }
}

fn part2(inp: &Input) -> Output {
    let mut multiverse = Multiverse::new(inp.0, inp.1);
    loop {
        for i in [0, 1] {
            multiverse.turn(i);
            if let Some(n) = multiverse.conclude() {
                return n;
            }
        }
    }
}

util::register_alt!(parse, part1, part2);
