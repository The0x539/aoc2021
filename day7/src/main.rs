type Input = Vec<i32>;
type Output = i32;

fn parse_input(s: &str) -> Input {
    s.split(',').map(|t| t.parse().unwrap()).collect()
}

fn part1(input: &[Input]) -> Output {
    let positions = input[0].clone();
    let min = positions.iter().copied().min().unwrap();
    let max = positions.iter().copied().max().unwrap();
    (min..=max)
        .map(|n| positions.iter().copied().map(|x| (x - n).abs()).sum())
        .min()
        .unwrap()
}

fn part2_cost(x: i32, n: i32) -> i32 {
    let a = (x - n).abs();
    a * (a + 1) / 2
}

fn part2(input: &[Input]) -> Output {
    let positions = input[0].clone();
    let min = positions.iter().copied().min().unwrap();
    let max = positions.iter().copied().max().unwrap();
    (min..=max)
        .map(|n| positions.iter().copied().map(|x| part2_cost(x, n)).sum())
        .min()
        .unwrap()
}

util::register!(parse_input, part1, part2);
