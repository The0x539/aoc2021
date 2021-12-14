use std::collections::HashMap;
type Rules = HashMap<(u8, u8), u8>;

struct Input {
    template: Vec<u8>,
    rules: Rules,
}

type Output = usize;

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

fn apply(state: Vec<u8>, rules: &Rules) -> Vec<u8> {
    let mut new_state = vec![];
    for i in 0..state.len() - 1 {
        new_state.push(state[i]);
        if let Some(insertion) = rules.get(&(state[i], state[i + 1])) {
            new_state.push(*insertion);
        }
    }
    new_state.push(*state.last().unwrap());
    new_state
}

fn part1(inp: &Input) -> Output {
    let mut state = inp.template.clone();
    for _ in 0..10 {
        state = apply(state, &inp.rules);
    }
    let mut frequencies = HashMap::<u8, usize>::new();
    for elem in state {
        *frequencies.entry(elem).or_default() += 1;
    }
    let min = frequencies.values().copied().min().unwrap();
    let max = frequencies.values().copied().max().unwrap();
    max - min
}

fn part2(inp: &Input) -> Output {
    let mut state = inp.template.clone();
    for i in 0..40 {
        println!("{}: {}", i, state.len());
        state = apply(state, &inp.rules);
    }
    let mut frequencies = HashMap::<u8, usize>::new();
    for elem in state {
        *frequencies.entry(elem).or_default() += 1;
    }
    let min = frequencies.values().copied().min().unwrap();
    let max = frequencies.values().copied().max().unwrap();
    max - min
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
    let (a, b) = util::parse_output::<usize>();
    let input = std::fs::read_to_string("test.txt").unwrap();
    let input = parse(&input);
    assert_eq!(a, part1(&input));
    assert_eq!(b, part2(&input));
}
