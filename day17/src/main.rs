#[derive(Debug)]
struct Input {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn sign(n: i32) -> i32 {
    match n {
        i32::MIN..=-1 => -1,
        0 => 0,
        1..=i32::MAX => 1,
    }
}

type Output = i32;

fn parse(s: &str) -> Input {
    let find = |c, n| s.match_indices(c).map(|x| x.0).nth(n).unwrap();

    let i1 = find('=', 0) + 1;
    let i2 = find('.', 0);
    let i3 = find('.', 1) + 1;
    let i4 = find(',', 0);
    let i5 = find('=', 1) + 1;
    let i6 = find('.', 2);
    let i7 = find('.', 3) + 1;
    let i8 = s.trim().len();

    Input {
        min_x: s[i1..i2].parse().unwrap(),
        max_x: s[i3..i4].parse().unwrap(),
        min_y: s[i5..i6].parse().unwrap(),
        max_y: s[i7..i8].parse().unwrap(),
    }
}

fn simulate(input: &Input, mut xv: i32, mut yv: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;

    let mut max_y = 0;

    loop {
        x += xv;
        y += yv;
        xv -= sign(xv);
        yv -= 1;

        max_y = max_y.max(y);

        if (input.min_x..=input.max_x).contains(&x) && (input.min_y..=input.max_y).contains(&y) {
            return Some(max_y);
        } else if xv == 0 && yv < 0 && y < input.min_y {
            return None;
        }
    }
}

fn part1(inp: &Input) -> Output {
    let mut max_y = i32::MIN;

    for x in -200..200 {
        for y in -200..200 {
            if let Some(v) = simulate(inp, x, y) {
                max_y = max_y.max(v);
            }
        }
    }

    max_y
}

fn part2(inp: &Input) -> Output {
    let mut count = 0;

    for x in -200..200 {
        for y in -200..200 {
            if let Some(_) = simulate(inp, x, y) {
                count += 1
            }
        }
    }

    count
}

util::register_alt!(parse, part1, part2);
