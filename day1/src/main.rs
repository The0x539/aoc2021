#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

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

util::register!(util::parse_unwrap::<u16>, part1, part2);
