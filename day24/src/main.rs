#![cfg_attr(test, feature(test))]

use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Var {
    X,
    Y,
    Z,
    W,
}

impl std::str::FromStr for Var {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s {
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            "w" => Var::W,
            _ => return Err(()),
        };
        Ok(v)
    }
}

#[derive(Copy, Clone)]
enum Rval {
    Var(Var),
    Imm(i32),
}

impl FromStr for Rval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = if let Ok(n) = s.parse::<i32>() {
            Rval::Imm(n)
        } else {
            Rval::Var(s.parse()?)
        };
        Ok(v)
    }
}

#[derive(Copy, Clone)]
enum Op {
    Inp(Var),
    Add(Var, Rval),
    Mul(Var, Rval),
    Div(Var, Rval),
    Mod(Var, Rval),
    Eql(Var, Rval),
}

impl Op {
    fn dest(&self) -> Var {
        use Op::*;
        match self {
            Inp(v) | Add(v, _) | Mul(v, _) | Div(v, _) | Mod(v, _) | Eql(v, _) => *v,
        }
    }

    fn val(&self) -> Rval {
        use Op::*;
        match self {
            Add(_, v) | Mul(_, v) | Div(_, v) | Mod(_, v) | Eql(_, v) => *v,
            Inp(_) => panic!(),
        }
    }
}

type Input = Op;
type Output = u128;

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let var = s[4..5].parse()?;
        let val = || s[6..].parse();
        let v = match &s[..3] {
            "inp" => Op::Inp(var),
            "add" => Op::Add(var, val()?),
            "mul" => Op::Mul(var, val()?),
            "div" => Op::Div(var, val()?),
            "mod" => Op::Mod(var, val()?),
            "eql" => Op::Eql(var, val()?),
            _ => return Err(()),
        };
        Ok(v)
    }
}

#[derive(Default, Copy, Clone)]
struct Machine<'a> {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    code: &'a [Op],
}

impl std::cmp::PartialEq for Machine<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}
impl std::cmp::Eq for Machine<'_> {}
impl std::hash::Hash for Machine<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.z.hash(state)
    }
}

impl<'a> Machine<'a> {
    fn new(code: &'a [Op]) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }

    fn reg(&mut self, v: Var) -> &mut i32 {
        match v {
            Var::X => &mut self.x,
            Var::Y => &mut self.y,
            Var::Z => &mut self.z,
            Var::W => &mut self.w,
        }
    }

    fn execute(&mut self, op: Op, input_val: &mut i32) {
        let val = if let Op::Inp(_) = op {
            std::mem::take(input_val)
        } else {
            match op.val() {
                Rval::Var(v) => *self.reg(v),
                Rval::Imm(v) => v,
            }
        };

        let dest = self.reg(op.dest());
        *dest = match op {
            Op::Inp(..) => val,
            Op::Add(..) => *dest + val,
            Op::Mul(..) => *dest * val,
            Op::Div(..) => *dest / val,
            Op::Mod(..) => *dest % val,
            Op::Eql(..) => (*dest == val) as i32,
        };
    }

    fn run(&mut self, mut input_val: i32) {
        while let Some(op) = self.code.get(0).copied() {
            if matches!(op, Op::Inp(..)) && input_val == 0 {
                // we need another input
                break;
            }
            self.execute(op, &mut input_val);
            self.code = &self.code[1..];
        }
    }
}

fn finalize(machine_input: &[i32]) -> u128 {
    let mut n = 0;
    for digit in machine_input {
        n = (n * 10) + *digit as u128;
    }
    n
}

fn run(inp: &[Input], mut comparator: impl FnMut(Vec<i32>, Vec<i32>) -> Vec<i32>) -> u128 {
    let mut states = HashMap::<Machine, Vec<i32>>::default();
    states.insert(Machine::new(inp), vec![]);

    for _ in 0..14 {
        let mut new_states = HashMap::default();

        for (machine, past_input) in states {
            for digit in 1..=9 {
                let mut new_input = past_input.clone();
                new_input.push(digit);

                let mut m = machine;
                m.run(digit);

                let entry = new_states.entry(m).or_insert(new_input.clone());
                *entry = comparator(entry.clone(), new_input);
            }
        }

        states = new_states;
    }

    for (machine, best_input) in states {
        assert!(machine.code.is_empty());
        if machine.z == 0 {
            return finalize(&best_input);
        }
    }
    panic!()
}

fn part1(inp: &[Input]) -> Output {
    run(inp, std::cmp::max)
}

fn part2(inp: &[Input]) -> Output {
    run(inp, std::cmp::min)
}

util::register!(util::parse_unwrap::<Op>, part1, part2);
