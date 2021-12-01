#![feature(array_windows)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(input: &str) -> Vec<u16> {
    BufReader::new(File::open(input).unwrap())
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}

fn part1(input: &[u16]) -> usize {
    input.iter().zip(&input[1..]).filter(|(a, b)| b > a).count()
}

fn part2(input: &[u16]) -> usize {
    let iter = input
        .array_windows::<3>()
        .map(|x| x.into_iter().sum::<u16>());

    iter.clone()
        .zip(iter.skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let input = parse("input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
fn parse_out(output: &str) -> (usize, usize) {
    let f = BufReader::new(File::open(output).unwrap());
    let mut l = f.lines();
    let mut n = || l.next().unwrap().unwrap().trim().parse().unwrap();
    (n(), n())
}

#[cfg(test)]
#[test]
fn test() {
    let input = parse("test.txt");
    let (a, b) = parse_out("test.out.txt");
    assert_eq!(part1(&input), a);
    assert_eq!(part2(&input), b);
}
