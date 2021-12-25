#![cfg_attr(test, feature(test))]

#[derive(PartialEq, Copy, Clone)]
enum Space {
    Empty,
    East,
    South,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Space::Empty => '.',
            Space::East => '>',
            Space::South => 'v',
        };
        write!(f, "{}", c)
    }
}

type Input = Vec<Space>;
type Output = i32;

fn parse(s: &str) -> Input {
    let mut v = Vec::with_capacity(s.len());
    for b in s.bytes() {
        v.push(match b {
            b'.' => Space::Empty,
            b'>' => Space::East,
            b'v' => Space::South,
            _ => panic!(),
        });
    }
    v
}

fn step(state: &[Vec<Space>]) -> Vec<Vec<Space>> {
    let h = state.len();
    let w = state[0].len();

    let get = |st: &[Vec<Space>], pos: (usize, usize)| st[pos.1][pos.0];
    let set = |st: &mut [Vec<Space>], pos: (usize, usize), val: Space| st[pos.1][pos.0] = val;

    let mut state2 = vec![vec![Space::Empty; w]; h];

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            if get(&state, pos) == Space::South {
                set(&mut state2, pos, Space::South);
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            if get(&state, pos) != Space::East {
                continue;
            }
            let dest = ((x + 1) % w, y);

            let new_pos = if get(&state, dest) == Space::Empty {
                dest
            } else {
                pos
            };
            set(&mut state2, new_pos, Space::East);
        }
    }

    let mut state3 = vec![vec![Space::Empty; w]; h];

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            if get(&state2, pos) == Space::East {
                set(&mut state3, pos, Space::East);
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            if get(&state2, pos) != Space::South {
                continue;
            }
            let dest = (x, (y + 1) % h);

            let new_pos = if get(&state2, dest) == Space::Empty {
                dest
            } else {
                pos
            };
            set(&mut state3, new_pos, Space::South);
        }
    }

    state3
}

fn part1(inp: &[Input]) -> Output {
    let mut state = inp.to_owned();

    for i in 1.. {
        let new_state = step(&state);
        if new_state == state {
            return i;
        }
        state = new_state;
    }

    panic!()
}

fn part2(_inp: &[Input]) -> Output {
    0
}

util::register!(parse, part1, part2);
