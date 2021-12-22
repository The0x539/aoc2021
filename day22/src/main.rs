#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;
use text_io::scan;

type Input = Cuboid;
type Output = i64;

fn parse(s: &str) -> Input {
    let (l, r) = s.split_once(' ').unwrap();

    let state = l == "on";

    let x0: i64;
    let x1: i64;
    let y0: i64;
    let y1: i64;
    let z0: i64;
    let z1: i64;

    scan!(r.bytes() => "x={}..{},y={}..{},z={}..{}", x0, x1, y0, y1, z0, z1);

    Cuboid {
        state,
        x: x0..=x1,
        y: y0..=y1,
        z: z0..=z1,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cuboid {
    state: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

fn part1(inp: &[Cuboid]) -> Output {
    let mut cubes = BTreeSet::new();
    for v in inp {
        for x in v.x.clone() {
            if !(-50..=50).contains(&x) {
                continue;
            }
            for y in v.y.clone() {
                if !(-50..=50).contains(&y) {
                    continue;
                }
                for z in v.z.clone() {
                    if !(-50..=50).contains(&z) {
                        continue;
                    }
                    if v.state {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    cubes.len() as i64
}

fn range_overlap(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    b.contains(a.start()) || b.contains(a.end()) || a.contains(b.start()) || a.contains(b.end())
}

#[derive(Debug, PartialEq, Clone)]
struct JustCuboid {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl std::fmt::Display for JustCuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (&self.x, &self.y, &self.z))
    }
}

impl From<Cuboid> for JustCuboid {
    fn from(c: Cuboid) -> Self {
        let Cuboid { x, y, z, .. } = c;
        Self { x, y, z }
    }
}

impl JustCuboid {
    fn overlaps(&self, other: &Self) -> bool {
        range_overlap(&self.x, &other.x)
            && range_overlap(&self.y, &other.y)
            && range_overlap(&self.z, &other.z)
    }

    fn split_x(&self, x_split: i64) -> [Self; 2] {
        let x_a = *self.x.start()..=x_split - 1;
        let x_b = x_split..=*self.x.end();
        [x_a, x_b].map(|x| Self { x, ..self.clone() })
    }

    fn split_y(&self, y_split: i64) -> [Self; 2] {
        let y_a = *self.y.start()..=y_split - 1;
        let y_b = y_split..=*self.y.end();
        [y_a, y_b].map(|y| Self { y, ..self.clone() })
    }

    fn split_z(&self, z_split: i64) -> [Self; 2] {
        let z_a = *self.z.start()..=z_split - 1;
        let z_b = z_split..=*self.z.end();
        [z_a, z_b].map(|z| Self { z, ..self.clone() })
    }

    fn volume(&self) -> i64 {
        let x = self.x.end() - self.x.start() + 1;
        let y = self.y.end() - self.y.start() + 1;
        let z = self.z.end() - self.z.start() + 1;
        x * y * z
    }

    fn with_state(self, state: bool) -> Cuboid {
        let Self { x, y, z } = self;
        Cuboid { state, x, y, z }
    }
}

fn part2(inp: &[Input]) -> Output {
    let mut to_insert = inp.to_vec();
    to_insert.reverse();
    let mut cuboids: Vec<JustCuboid> = vec![];

    'insert: while let Some(v) = to_insert.pop() {
        //println!("{} {}", to_insert.len(), cuboids.len());
        let cub = JustCuboid::from(v.clone());

        let mut match_idx = None;
        'find_match: loop {
            macro_rules! push {
                ($cub:expr, $cs:expr => old) => {
                    let cs = $cs;
                    // println!("old {} -> {},{}", $cub, cs[0], cs[1]);
                    cuboids.extend(cs);
                    continue 'find_match;
                };
                ($cub:expr, $cs:expr => new) => {
                    let cs = $cs;
                    // println!("new {} -> {},{}", $cub, cs[0], cs[1]);
                    to_insert.extend(cs.map(|foo| foo.with_state(v.state)));
                    continue 'insert;
                };
            }

            for i in (0..cuboids.len()).rev() {
                let c = &cuboids[i];
                if c.overlaps(&cub) {
                    if c == &cub {
                        match_idx = Some(i);
                        break 'find_match;
                    }

                    // println!("overlap {}, {}", cub, c);

                    let comparisons = [
                        (c.x.start(), cub.x.start()),
                        (c.x.end(), cub.x.end()),
                        (c.y.start(), cub.y.start()),
                        (c.y.end(), cub.y.end()),
                        (c.z.start(), cub.z.start()),
                        (c.z.end(), cub.z.end()),
                    ]
                    .map(|(a, b)| a.cmp(b));

                    use Ordering::*;
                    match comparisons {
                        [Less, _, _, _, _, _] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_x(*cub.x.start());
                            push!(c, cs => old);
                        }
                        [Greater, _, _, _, _, _] => {
                            let cs = cub.split_x(*c.x.start());
                            push!(cub, cs => new);
                        }
                        [_, Less, _, _, _, _] => {
                            let cs = cub.split_x(*c.x.end() + 1);
                            push!(cub, cs => new);
                        }
                        [_, Greater, _, _, _, _] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_x(*cub.x.end() + 1);
                            push!(c, cs => old);
                        }
                        [_, _, Less, _, _, _] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_y(*cub.y.start());
                            push!(c, cs => old);
                        }
                        [_, _, Greater, _, _, _] => {
                            let cs = cub.split_y(*c.y.start());
                            push!(cub, cs => new);
                        }
                        [_, _, _, Less, _, _] => {
                            let cs = cub.split_y(*c.y.end() + 1);
                            push!(cub, cs => new);
                        }
                        [_, _, _, Greater, _, _] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_y(*cub.y.end() + 1);
                            push!(c, cs => old);
                        }
                        [_, _, _, _, Less, _] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_z(*cub.z.start());
                            push!(c, cs => old);
                        }
                        [_, _, _, _, Greater, _] => {
                            let cs = cub.split_z(*c.z.start());
                            push!(cub, cs => new);
                        }
                        [_, _, _, _, _, Less] => {
                            let cs = cub.split_z(*c.z.end() + 1);
                            push!(cub, cs => new);
                        }
                        [_, _, _, _, _, Greater] => {
                            let c = cuboids.swap_remove(i);
                            let cs = c.split_z(*cub.z.end() + 1);
                            push!(c, cs => old);
                        }
                        _ => todo!(),
                    }
                }
            }
            break 'find_match;
        }

        if v.state {
            if match_idx.is_none() {
                cuboids.push(cub);
            }
        } else {
            if let Some(i) = match_idx {
                cuboids.swap_remove(i);
            }
        }
    }
    cuboids.iter().map(JustCuboid::volume).sum()
}

util::register!(parse, part1, part2);
