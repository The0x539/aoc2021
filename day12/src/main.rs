use std::collections::{HashMap, HashSet};

struct Input(String, String);
type Output = usize;

fn is_lowercase(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_lowercase())
}

fn parse(s: &str) -> Input {
    let (a, b) = s.split_once("-").unwrap();
    Input(a.to_owned(), b.to_owned())
}

#[derive(Clone)]
struct Path {
    location: String,
    history: HashSet<String>,
    can_repeat: bool,
}

impl Path {
    fn new(can_repeat: bool) -> Self {
        Self {
            location: "start".to_owned(),
            history: std::iter::once("start".to_owned()).collect(),
            can_repeat,
        }
    }

    fn visit(&mut self, dst: &str) {
        self.location = dst.to_owned();
        self.history.insert(dst.to_owned());
    }
}

fn run(input: &[Input], can_repeat: bool) -> Output {
    let connectivity = {
        let mut c = HashMap::<String, HashSet<String>>::new();
        for Input(a, b) in input {
            c.entry(a.clone()).or_default().insert(b.clone());
            c.entry(b.clone()).or_default().insert(a.clone());
        }
        c
    };

    let mut incomplete_paths = vec![Path::new(can_repeat)];
    let mut num_complete_paths = 0;

    while let Some(path) = incomplete_paths.pop() {
        for dst in &connectivity[&path.location] {
            if dst == "start" {
                continue;
            } else if dst == "end" {
                num_complete_paths += 1;
                continue;
            }

            let mut new_path;

            if is_lowercase(dst) && path.history.contains(dst) {
                if path.can_repeat {
                    new_path = path.clone();
                    new_path.can_repeat = false;
                } else {
                    continue;
                }
            } else {
                new_path = path.clone();
            }

            new_path.visit(dst);
            incomplete_paths.push(new_path);
        }
    }

    num_complete_paths
}

fn part1(input: &[Input]) -> Output {
    run(input, false)
}

fn part2(input: &[Input]) -> Output {
    run(input, true)
}

util::register!(parse, part1, part2);
