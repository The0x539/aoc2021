#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

type Input = Vec<u8>;
type Output = usize;

fn parse(s: &str) -> Input {
    s.trim().bytes().map(|b| b - b'0').collect()
}

fn simulate(state: &mut [Input]) -> usize {
    let h = state.len();
    let w = state[0].len();

    let mut flashed = HashSet::new();
    let mut flash_stack = vec![];
    for y in 0..h {
        for x in 0..w {
            state[y][x] += 1;
            if state[y][x] > 9 {
                flash_stack.push((x, y));
            }
        }
    }

    while let Some((x, y)) = flash_stack.pop() {
        if flashed.contains(&(x, y)) {
            continue;
        }
        flashed.insert((x, y));
        for (xx, yy) in util::oct_neighbors(x, y, w, h) {
            state[yy][xx] += 1;
            if state[yy][xx] > 9 {
                flash_stack.push((xx, yy));
            }
        }
    }

    let count = flashed.len();
    for (x, y) in flashed {
        state[y][x] = 0;
    }
    count
}

fn part1(inp: &[Input]) -> Output {
    let mut total = 0;
    let mut state = inp.to_vec();
    for _ in 0..100 {
        total += simulate(&mut state);
    }
    total
}

fn part2(inp: &[Input]) -> Output {
    let mut state = inp.to_vec();
    let num_octopodes = state.len() * state[0].len();

    for i in 1.. {
        if simulate(&mut state) == num_octopodes {
            return i;
        }
    }
    panic!("oh no");
}

util::register!(parse, part1, part2);
