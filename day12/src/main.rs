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

fn part1(input: &[Input]) -> Output {
    let connectivity = {
        let mut c = HashMap::<String, HashSet<String>>::new();
        for Input(a, b) in input {
            c.entry(a.clone()).or_default().insert(b.clone());
            c.entry(b.clone()).or_default().insert(a.clone());
        }
        c
    };

    let mut paths = vec![vec!["start".to_owned()]];

    loop {
        let mut is_done = true;
        let mut new_paths = vec![];

        for path in paths {
            let last = path.last().unwrap();
            if last == "end" {
                new_paths.push(path);
                continue;
            }
            for dst in &connectivity[last] {
                if is_lowercase(dst) && path.contains(dst) {
                    continue;
                }
                is_done = false;
                let mut new_path = path.clone();
                new_path.push(dst.clone());
                new_paths.push(new_path);
            }
        }

        paths = new_paths;
        if is_done {
            break;
        }
    }
    paths.len()
}

fn part2(input: &[Input]) -> Output {
    let connectivity = {
        let mut c = HashMap::<String, HashSet<String>>::new();
        for Input(a, b) in input {
            c.entry(a.clone()).or_default().insert(b.clone());
            c.entry(b.clone()).or_default().insert(a.clone());
        }
        c
    };

    let mut paths = vec![(vec!["start".to_owned()], false)];

    loop {
        let mut is_done = true;
        let mut new_paths = vec![];

        for path in paths {
            let last = path.0.last().unwrap();
            if last == "end" {
                new_paths.push(path);
                continue;
            }
            for dst in &connectivity[last] {
                let mut new_path = path.clone();
                if is_lowercase(dst) && path.0.contains(dst) {
                    if path.1 || dst == "start" {
                        continue;
                    } else {
                        new_path.1 = true;
                    }
                }
                is_done = false;
                new_path.0.push(dst.clone());
                new_paths.push(new_path);
            }
        }

        paths = new_paths;
        if is_done {
            break;
        }
    }

    paths.len()
}

util::register!(parse, part1, part2);
