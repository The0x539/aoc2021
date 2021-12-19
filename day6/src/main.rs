#![cfg_attr(test, feature(test))]

type State = [u64; 9];

fn parse_input(s: &str) -> State {
    let mut state = [0; 9];
    for n in s.split(",").map(|t| t.parse::<usize>().unwrap()) {
        state[n] += 1;
    }
    state
}

fn run(mut state: State, time: u16) -> u64 {
    for _ in 0..time {
        let mut new_state = [0; 9];
        for i in 1..=8 {
            new_state[i - 1] = state[i];
        }
        new_state[6] += state[0];
        new_state[8] += state[0];
        state = new_state;
    }
    state.into_iter().sum()
}

fn part1(input: &[State]) -> u64 {
    run(input[0], 80)
}

fn part2(input: &[State]) -> u64 {
    run(input[0], 256)
}

util::register!(parse_input, part1, part2);
