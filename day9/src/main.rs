use std::collections::HashMap;

type Input = Vec<u8>;
type Output = u32;

fn parse(s: &str) -> Input {
    s.bytes().map(|c| c - b'0').collect()
}

fn part1(input: &[Input]) -> Output {
    let mut total = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let v = input[y][x];
            if (x > 0 && input[y][x - 1] <= v)
                || (x < input[0].len() - 1 && input[y][x + 1] <= v)
                || (y > 0 && input[y - 1][x] <= v)
                || (y < input.len() - 1 && input[y + 1][x] <= v)
            {
                continue;
            }
            total += v as u32 + 1;
        }
    }
    total
}

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter_map(|(x, y)| Some((usize::try_from(x).ok()?, usize::try_from(y).ok()?)))
        .filter(move |&(x, y)| x < width && y < height)
}

fn part2(input: &[Input]) -> Output {
    let mut basins = HashMap::<(usize, usize), Vec<(usize, usize)>>::new();
    let (width, height) = (input[0].len(), input.len());
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == 9 {
                continue;
            }

            let (mut low_x, mut low_y) = (x, y);
            while let Some((new_low_x, new_low_y)) = neighbors(low_x, low_y, width, height)
                .filter(|&(x, y)| input[y][x] < input[low_y][low_x])
                .next()
            {
                low_x = new_low_x;
                low_y = new_low_y;
            }

            basins.entry((low_x, low_y)).or_default().push((x, y));
        }
    }

    let mut sizes = basins.values().map(Vec::len).collect::<Vec<_>>();
    sizes.sort();
    sizes.iter().rev().map(|&x| x as u32).take(3).product()
}

util::register!(parse, part1, part2);
