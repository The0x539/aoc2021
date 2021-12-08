use itertools::Itertools;
use std::str::FromStr;

#[derive(Default, Copy, Clone)]
struct SignalPattern([bool; 7]);

impl SignalPattern {
    const PATTERNS: [[u8; 7]; 10] = [
        [1, 1, 1, 0, 1, 1, 1],
        [0, 0, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 1, 0, 1],
        [1, 0, 1, 1, 0, 1, 1],
        [0, 1, 1, 1, 0, 1, 0],
        [1, 1, 0, 1, 0, 1, 1],
        [1, 1, 0, 1, 1, 1, 1],
        [1, 0, 1, 0, 0, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 1],
    ];

    fn count(&self) -> u8 {
        self.0.iter().map(|n| *n as u8).sum()
    }

    fn is_unique_digit(&self) -> bool {
        [2, 4, 3, 7].contains(&self.count())
    }

    fn is_sanely_valid(&self) -> bool {
        self.digit().is_some()
    }

    fn digit(&self) -> Option<u8> {
        let p = self.0.map(|x| x as u8);
        for i in 0..=9 {
            if Self::PATTERNS[i as usize] == p {
                return Some(i);
            }
        }
        None
    }
}

impl FromStr for SignalPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::default();
        for c in s.bytes() {
            let i = match c {
                b'a'..=b'g' => c - b'a',
                _ => return Err("bad char"),
            };
            this.0[i as usize] = true;
        }
        Ok(this)
    }
}

#[derive(Default)]
struct Input([SignalPattern; 10], [SignalPattern; 4]);

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::default();
        let mut vals = s.split(' ').filter(|&s| s != "|");
        for (i, v) in vals.by_ref().take(10).enumerate() {
            this.0[i] = v.parse()?;
        }
        for (i, v) in vals.enumerate() {
            this.1[i] = v.parse()?;
        }
        Ok(this)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Mapping([usize; 7]);

impl Mapping {
    fn map(&self, sig: &SignalPattern) -> SignalPattern {
        let mut out = SignalPattern::default();
        for i in 0..7 {
            out.0[self.0[i]] = sig.0[i];
        }
        out
    }

    fn valid_for(&self, sigs: &[SignalPattern; 10]) -> bool {
        sigs.iter().all(|sig| self.map(sig).is_sanely_valid())
    }

    fn determine(sigs: &[SignalPattern; 10]) -> impl Iterator<Item = Self> + '_ {
        (0..7)
            .permutations(7)
            .map(|v| Self(v.try_into().unwrap()))
            .filter(|candidate| candidate.valid_for(sigs))
    }

    fn output(&self, sigs: &[SignalPattern; 4]) -> u32 {
        let [a, b, c, d] = sigs.map(|sig| self.map(&sig).digit().unwrap() as u32);
        a * 1000 + b * 100 + c * 10 + d
    }

    fn full_affair(input: &Input) -> u32 {
        Self::determine(&input.0)
            .exactly_one()
            .ok()
            .unwrap()
            .output(&input.1)
    }
}

#[cfg(test)]
#[test]
fn mini_test() {
    let input_str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    let input = input_str.parse::<Input>().unwrap();

    let ms = Mapping::determine(&input.0).collect_vec();
    assert_eq!(ms, vec![Mapping([2, 5, 6, 0, 1, 3, 4])]);

    let m = ms[0];
    assert_eq!(m.output(&input.1), 5353);
}

type Output = u32;

fn part1(input: &[Input]) -> Output {
    input
        .iter()
        .flat_map(|x| &x.1)
        .filter(|s| s.is_unique_digit())
        .count() as u32
}

fn part2(input: &[Input]) -> Output {
    input.iter().map(Mapping::full_affair).sum()
}

util::register!(util::parse_unwrap::<Input>, part1, part2);
