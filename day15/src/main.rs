#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

type Input = Vec<u8>;
type Output = u64;

fn parse(s: &str) -> Input {
    s.bytes().map(|c| c - b'0').collect()
}

fn dijkstra(costs: Vec<Vec<u8>>) -> Output {
    let h = costs.len();
    let w = costs[0].len();

    let mut distances = vec![vec![u64::MAX; w]; h];
    distances[0][0] = 0;

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert((0, 0));

    let mut to_visit = HashSet::new();

    let (mut x, mut y) = (0, 0);

    loop {
        for (nx, ny) in util::quad_neighbors(x, y, w, h) {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            let potential = distances[y][x] + costs[ny][nx] as u64;
            if potential < distances[ny][nx] {
                distances[ny][nx] = potential;
                to_visit.insert((nx, ny));
            }
        }
        to_visit.remove(&(x, y));
        visited.insert((x, y));

        let (cx, cy) = to_visit
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

    dijkstra(costs)
}

util::register!(parse, part1, part2);
