#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use std::collections::HashMap;
type Rules = HashMap<(u8, u8), u8>;

struct Input {
    template: Vec<u8>,
    rules: Rules,
}

type Output = u64;

fn parse(s: &str) -> Input {
    let mut lines = s.lines();

    let template = lines.next().unwrap().trim().as_bytes().to_vec();
    let mut rules = Rules::new();
    lines.next();
    for line in lines {
        let line = line.as_bytes();
        let (a, b, c) = (line[0], line[1], line[6]);
        rules.insert((a, b), c);
    }
    Input { template, rules }
}

#[derive(Default)]
struct State {
    pair_counts: HashMap<(u8, u8), u64>,
    char_counts: HashMap<u8, u64>,
}

impl State {
    fn new(template: &[u8]) -> Self {
        let mut state = Self::default();
        for &[a, b] in template.array_windows() {
            state.insert_pair(a, b, 1);
        }
        for &c in template {
            state.insert_char(c, 1);
        }
        state
    }

    fn apply(&mut self, rules: &Rules) {
        for ((a, b), n) in self.pair_counts.clone() {
            if let Some(&c) = rules.get(&(a, b)) {
                self.remove_pair(a, b, n);
                self.insert_pair(a, c, n);
                self.insert_pair(c, b, n);
                self.insert_char(c, n);
            } else {
                println!("no insertion"); // this never happens
            }
        }
    }

    fn remove_pair(&mut self, a: u8, b: u8, n: u64) {
        *self.pair_counts.get_mut(&(a, b)).unwrap() -= n;
    }

    fn insert_pair(&mut self, a: u8, b: u8, n: u64) {
        *self.pair_counts.entry((a, b)).or_default() += n;
    }

    fn insert_char(&mut self, a: u8, n: u64) {
        *self.char_counts.entry(a).or_default() += n;
    }

    fn result(&self) -> u64 {
        let max = self.char_counts.values().max().unwrap();
        let min = self.char_counts.values().min().unwrap();
        max - min
    }
}

fn run(inp: &Input, steps: u8) -> Output {
    let mut state = State::new(&inp.template);
    for _ in 0..steps {
        state.apply(&inp.rules);
    }
    state.result()
}

fn part1(inp: &Input) -> Output {
    run(inp, 10)
}

fn part2(inp: &Input) -> Output {
    run(inp, 40)
}

util::register_alt!(parse, part1, part2);
