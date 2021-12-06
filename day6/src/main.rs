use std::collections::BTreeMap;

type State = BTreeMap<u8, u64>;

fn make_state() -> State {
    (0..=8).zip([0; 9]).collect()
}

fn parse_input(s: &str) -> State {
    let mut state = make_state();
    for n in s.split(",").map(|t| t.parse().unwrap()) {
        *state.entry(n).or_default() += 1;
    }
    state
}

fn run(mut state: State, time: u16) -> u64 {
    for _ in 0..time {
        let mut new_state = make_state();
        for i in 1..=8 {
            new_state.insert(i - 1, state[&i]);
        }
        let n_zero = state[&0];
        *new_state.get_mut(&8).unwrap() += n_zero;
        *new_state.get_mut(&6).unwrap() += n_zero;
        state = new_state;
    }
    state.values().sum()
}

fn part1(input: &[State]) -> u64 {
    run(input[0].clone(), 80)
}

fn part2(input: &[State]) -> u64 {
    run(input[0].clone(), 256)
}

util::register!(parse_input, part1, part2);
