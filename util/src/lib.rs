use std::fmt::{Debug, Display};
use std::path::Path;
use std::str::FromStr;

pub fn parse_unwrap<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    s.trim().parse().unwrap()
}

pub fn parse_input<P: AsRef<Path>, T, F: FnMut(&str) -> T>(input_path: P, mut f: F) -> Vec<T> {
    std::fs::read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| f(line.trim()))
        .collect()
}

pub fn run<Parser, Part1, Part2, In, Out>(parser: Parser, part1: Part1, part2: Part2)
where
    Parser: FnMut(&str) -> In,
    Part1: FnOnce(&[In]) -> Out,
    Part2: FnOnce(&[In]) -> Out,
    Out: Display,
{
    let input = parse_input("input.txt", parser);
    println!("{}\n{}", part1(&input), part2(&input));
}

pub fn run_alt<Parser, Part1, Part2, In, Out>(parser: Parser, part1: Part1, part2: Part2)
where
    Parser: FnOnce(&str) -> In,
    Part1: FnOnce(&In) -> Out,
    Part2: FnOnce(&In) -> Out,
    Out: Display,
{
    let input_data = std::fs::read_to_string("input.txt").unwrap();
    let input = parser(&input_data);
    println!("{}\n{}", part1(&input), part2(&input));
}

pub fn test<Parser, Part1, Part2, In, Out>(parser: Parser, part1: Part1, part2: Part2)
where
    Parser: FnMut(&str) -> In,
    Part1: FnOnce(&[In]) -> Out,
    Part2: FnOnce(&[In]) -> Out,
    Out: Debug + FromStr + PartialEq,
    Out::Err: Debug,
{
    let input = parse_input("test.txt", parser);
    let (x, y) = parse_output::<Out>();

    assert_eq!(part1(&input), x);
    assert_eq!(part2(&input), y);
}

pub fn test_alt<Parser, Part1, Part2, In, Out>(parser: Parser, part1: Part1, part2: Part2)
where
    Parser: FnOnce(&str) -> In,
    Part1: FnOnce(&In) -> Out,
    Part2: FnOnce(&In) -> Out,
    Out: Debug + FromStr + PartialEq,
    Out::Err: Debug,
{
    let input_data = std::fs::read_to_string("test.txt").unwrap();
    let input = parser(&input_data);
    let (x, y) = parse_output::<Out>();

    assert_eq!(part1(&input), x);
    assert_eq!(part2(&input), y);
}

pub fn parse_output<T>() -> (T, T)
where
    T: FromStr,
    T::Err: Debug,
{
    let output_data = std::fs::read_to_string("test.out.txt").unwrap();
    let (a, b) = output_data.split_once("\n").unwrap();
    let [x, y] = [a, b].map(parse_unwrap::<T>);
    (x, y)
}

#[macro_export]
macro_rules! register {
    ($parser:expr, $part1:expr, $part2:expr) => {
        fn main() {
            $crate::run($parser, $part1, $part2);
        }

        #[cfg(test)]
        #[test]
        fn test() {
            $crate::test($parser, $part1, $part2);
        }
    };
}

#[macro_export]
macro_rules! register_alt {
    ($parser:expr, $part1:expr, $part2:expr) => {
        fn main() {
            $crate::run_alt($parser, $part1, $part2);
        }

        #[cfg(test)]
        #[test]
        fn test() {
            $crate::test_alt($parser, $part1, $part2);
        }
    };
}

pub fn quad_neighbors(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(move |&(x, y)| x < w && y < h)
}

pub fn oct_neighbors(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| x >= 0 && y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(move |&(x, y)| x < w && y < h)
}
