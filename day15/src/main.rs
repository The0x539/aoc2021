use std::collections::HashSet;

type Input = Vec<u8>;
type Output = u64;

fn parse(s: &str) -> Input {
    s.bytes().map(|c| c - b'0').collect()
}

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(move |&(x, y)| x < w && y < h)
}

fn dijkstra(costs: Vec<Vec<u8>>) -> Output {
    let h = costs.len();
    let w = costs[0].len();

    let mut distances = vec![vec![u64::MAX; w]; h];
    distances[0][0] = 0;

    let mut unvisited = HashSet::<(usize, usize)>::new();
    for x in 0..w {
        for y in 0..h {
            unvisited.insert((x, y));
        }
    }
    unvisited.remove(&(0, 0));

    let (mut x, mut y) = (0, 0);

    loop {
        for (nx, ny) in neighbors(x, y, w, h) {
            if !unvisited.contains(&(nx, ny)) {
                continue;
            }
            distances[ny][nx] = distances[ny][nx].min(distances[y][x] + costs[ny][nx] as u64);
        }
        unvisited.remove(&(x, y));

        let (cx, cy) = unvisited
            .iter()
            .copied()
            .min_by_key(|&(x, y)| distances[y][x])
            .unwrap();

        if (cx, cy) == (w - 1, h - 1) {
            break;
        } else {
            x = cx;
            y = cy;
        }
    }

    distances[h - 1][w - 1]
}

fn part1(costs: &[Input]) -> Output {
    dijkstra(costs.to_vec())
}

fn part2(inp: &[Input]) -> Output {
    let mut costs = Vec::new();
    let wrap = |x| match x {
        0..=9 => x,
        10.. => x - 9,
    };

    for yi in 0..5 {
        for small_row in inp {
            let mut big_row = Vec::new();
            for xi in 0..5 {
                big_row.extend(small_row.iter().copied().map(|v| wrap(v + xi + yi)));
            }
            costs.push(big_row);
        }
    }

    // Look, brute force got #957. I don't make the rules.
    dijkstra(costs)
}

util::register!(parse, part1, part2);
