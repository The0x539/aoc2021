#![cfg_attr(test, feature(test))]

type Input = Vec<char>;
type Output = u64;
use either::*;

fn parse(s: &str) -> Input {
    s.chars().collect()
}

fn check(line: &Input) -> Either<Output, Vec<char>> {
    let mut stack = vec![];
    for c in line {
        match *c {
            '(' | '[' | '{' | '<' => stack.push(*c),
            ')' => {
                if stack.pop() != Some('(') {
                    return Left(3);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Left(57);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Left(1197);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Left(25137);
                }
            }
            _ => panic!("?"),
        }
    }
    Right(stack)
}

fn part1(inp: &[Input]) -> Output {
    inp.iter().map(check).flat_map(Either::left).sum()
}

fn part2(inp: &[Input]) -> Output {
    let mut scores = vec![];
    for stack in inp.iter().map(check).flat_map(Either::right) {
        let mut score = 0;
        for c in stack.into_iter().rev() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("?"),
            };
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}

util::register!(parse, part1, part2);
