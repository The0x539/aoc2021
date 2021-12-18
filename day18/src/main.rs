#[derive(Debug, Copy, Clone)]
struct Val {
    value: u8,
    depth: u8,
}

#[derive(Debug, Clone)]
struct State {
    vals: Vec<Val>,
}

fn magnitude_rec(vals: &mut &[Val], depth: u8) -> Output {
    if vals[0].depth >= depth {
        3 * magnitude_rec(vals, depth + 1) + 2 * magnitude_rec(vals, depth + 1)
    } else {
        let v = vals[0].value;
        *vals = &vals[1..];
        v as Output
    }
}

type Input = State;
type Output = u32;

fn parse(s: &str) -> Input {
    let mut depth = 1;
    let mut input = State { vals: vec![] };
    for c in s[1..].bytes() {
        match c {
            b'0'..=b'9' => {
                let v = Val {
                    value: c - b'0',
                    depth: depth - 1,
                };
                input.vals.push(v);
            }
            b'[' => depth += 1,
            b']' => depth -= 1,
            b',' => (),
            _ => panic!("unexpected"),
        }
    }
    assert_eq!(depth, 0);
    input
}

impl State {
    fn add(&self, other: &Self) -> Self {
        let mut new = Self { vals: vec![] };
        for val in self.vals.iter().chain(&other.vals) {
            new.vals.push(Val {
                value: val.value,
                depth: val.depth + 1,
            });
        }
        new
    }

    fn try_explode(&mut self) -> bool {
        for j in 1..self.vals.len() {
            let i = j - 1;
            if self.vals[i].depth == 4 && self.vals[j].depth == 4 {
                if i > 0 {
                    self.vals[i - 1].value += self.vals[i].value;
                }
                if j < self.vals.len() - 1 {
                    self.vals[j + 1].value += self.vals[j].value;
                }
                self.vals[i].value = 0;
                self.vals[i].depth -= 1;
                self.vals.remove(j);
                return true;
            }
        }
        false
    }

    fn try_split(&mut self) -> bool {
        for i in 0..self.vals.len() {
            let v = self.vals[i].value;
            if v >= 10 {
                let d = self.vals[i].depth + 1;

                let l = v / 2;
                let r = l + (v & 1);

                self.vals[i].depth = d;
                self.vals[i].value = r;
                self.vals.insert(i, Val { value: l, depth: d });
                return true;
            }
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.try_explode() {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> Output {
        magnitude_rec(&mut &self.vals[..], 0)
    }
}

fn part1(inp: &[Input]) -> Output {
    let mut val = inp[0].clone();
    val.reduce();
    for addend in &inp[1..] {
        val = State::add(&val, addend);
        val.reduce();
    }
    val.magnitude()
}

fn part2(inp: &[Input]) -> Output {
    let mut n = 0;
    for a in inp {
        for b in inp {
            if a as *const _ != b as *const _ {
                n = n.max(part1(&[a.clone(), b.clone()]));
            }
        }
    }
    n
}

util::register!(parse, part1, part2);
