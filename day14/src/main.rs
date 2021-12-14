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

#[allow(dead_code)]
fn apply(state: Vec<u8>, rules: &Rules) -> Vec<u8> {
    let mut new_state = vec![];
    for i in 0..state.len() - 1 {
        new_state.push(state[i]);
        if let Some(insertion) = rules.get(&(state[i], state[i + 1])) {
            new_state.push(*insertion);
        } else {
            println!("didn't insert");
        }
    }
    new_state.push(*state.last().unwrap());
    new_state
}

#[allow(dead_code)]
fn part1_old(inp: &Input) -> Output {
    let mut state = inp.template.clone();
    for _ in 0..10 {
        state = apply(state, &inp.rules);
    }
    let mut frequencies = HashMap::<u8, u64>::new();
    for elem in state {
        *frequencies.entry(elem).or_default() += 1;
    }
    let min = frequencies.values().copied().min().unwrap();
    let max = frequencies.values().copied().max().unwrap();
    max - min
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
            state.insert_char(a, 1);
            state.insert_char(b, 1);
        }
        state
    }

    fn apply(&self, rules: &Rules) -> Self {
        let mut new_state = Self::default();
        new_state.char_counts = self.char_counts.clone();
        for (&(a, b), &n) in &self.pair_counts {
            if let Some(&c) = rules.get(&(a, b)) {
                new_state.insert_pair(a, c, n);
                new_state.insert_pair(c, b, n);
                new_state.insert_char(c, n);
            } else {
                println!("no insertion");
                new_state.insert_pair(a, b, n);
            }
        }

        new_state
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
        (max - min) + 2 // ??????????
    }
}

fn run(inp: &Input, steps: u8) -> Output {
    let mut state = State::new(&inp.template);
    for _ in 0..steps {
        state = state.apply(&inp.rules);
    }
    state.result()
}

fn part1(inp: &Input) -> Output {
    run(inp, 10)
}

fn part2(inp: &Input) -> Output {
    run(inp, 40)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
#[test]
fn test() {
    let (a, b) = util::parse_output::<u64>();
    let input = std::fs::read_to_string("test.txt").unwrap();
    let input = parse(&input);
    assert_eq!(a, part1(&input));
    assert_eq!(b, part2(&input));
}
