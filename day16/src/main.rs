#![cfg_attr(test, feature(test))]

use std::ops::{BitOr, Shl};

type Input = Vec<bool>;
type Output = u64;

fn parse(s: &str) -> Input {
    s.trim()
        .matches(|_| true)
        .flat_map(|c| {
            let n = u8::from_str_radix(c, 16).unwrap();
            [n & 8, n & 4, n & 2, n & 1].map(|v| v != 0)
        })
        .collect()
}

struct PacketHeader {
    version: u8,
    ty: u8,
}

pub trait FromBits {
    fn from_bits(bits: &[bool]) -> Self;
}

impl<T: From<bool> + BitOr<T, Output = T> + Shl<u8, Output = T>> FromBits for T {
    fn from_bits(bits: &[bool]) -> T {
        let mut val = T::from(false);
        for &bit in bits {
            val = (val << 1) | T::from(bit);
        }
        val
    }
}

fn read_bits<T: FromBits>(data: &mut &[bool], n: usize) -> T {
    let (left, right) = data.split_at(n);
    *data = right;
    T::from_bits(left)
}

fn read_packet_header(data: &mut &[bool]) -> PacketHeader {
    PacketHeader {
        version: read_bits(data, 3),
        ty: read_bits(data, 3),
    }
}

#[derive(Debug)]
enum Length {
    Bits(u16),
    Packets(u16),
}

fn read_length(data: &mut &[bool]) -> Length {
    let length_type_id = data[0];
    *data = &data[1..];
    if length_type_id {
        Length::Packets(read_bits(data, 11))
    } else {
        Length::Bits(read_bits(data, 15))
    }
}

fn read_number(data: &mut &[bool]) -> u64 {
    let mut bits = Vec::new();
    loop {
        let (left, right) = data.split_at(5);
        *data = right;
        let group: [bool; 5] = left.try_into().unwrap();

        bits.extend(group[1..].iter().copied());

        if !group[0] {
            break;
        }
    }

    assert!(bits.len() < 64);
    u64::from_bits(&bits)
}

enum Body {
    Value(u64),
    Children(Vec<Packet>),
}

fn read_body(data: &mut &[bool], ty: u8) -> Body {
    match ty {
        4 => Body::Value(read_number(data)),
        _ => {
            let mut children = vec![];
            match read_length(data) {
                Length::Bits(n) => {
                    let (mut my_data, right) = data.split_at(n as usize);
                    *data = right;
                    while !my_data.is_empty() {
                        children.push(read_packet(&mut my_data));
                    }
                }
                Length::Packets(n) => {
                    for _ in 0..n {
                        children.push(read_packet(data));
                    }
                }
            }
            Body::Children(children)
        }
    }
}

struct Packet {
    header: PacketHeader,
    body: Body,
}

fn read_packet(data: &mut &[bool]) -> Packet {
    let header = read_packet_header(data);
    let body = read_body(data, header.ty);
    Packet { header, body }
}

impl Packet {
    fn version_sum(&self) -> u64 {
        let mut sum = self.header.version as u64;
        if let Body::Children(children) = &self.body {
            for child in children {
                sum += child.version_sum();
            }
        }
        sum
    }
}

enum Part2Packet {
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Min(Vec<Self>),
    Max(Vec<Self>),
    Value(u64),
    Gt(Vec<Self>),
    Lt(Vec<Self>),
    Eq(Vec<Self>),
}

impl From<Packet> for Part2Packet {
    fn from(p: Packet) -> Self {
        if p.header.ty == 4 {
            let val = match p.body {
                Body::Value(n) => n,
                _ => panic!("not a value"),
            };
            return Self::Value(val);
        }

        let children = match p.body {
            Body::Children(cs) => cs.into_iter().map(Self::from).collect(),
            _ => panic!("shouldn't be a value"),
        };
        let variant = match p.header.ty {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("unexpected type"),
        };
        variant(children)
    }
}

impl Part2Packet {
    fn eval(&self) -> u64 {
        match self {
            Self::Sum(v) => v.iter().map(Self::eval).sum(),
            Self::Product(v) => v.iter().map(Self::eval).product(),
            Self::Min(v) => v.iter().map(Self::eval).min().unwrap(),
            Self::Max(v) => v.iter().map(Self::eval).max().unwrap(),
            Self::Value(n) => *n,
            Self::Gt(v) => (v[0].eval() > v[1].eval()) as u64,
            Self::Lt(v) => (v[0].eval() < v[1].eval()) as u64,
            Self::Eq(v) => (v[0].eval() == v[1].eval()) as u64,
        }
    }
}

fn part1(inp: &Input) -> Output {
    let mut data = inp.as_slice();
    let pkt = read_packet(&mut data);
    pkt.version_sum()
}

fn part2(inp: &Input) -> Output {
    let mut data = inp.as_slice();
    let pkt = read_packet(&mut data);
    let pkt = Part2Packet::from(pkt);
    pkt.eval()
}

fn main() {
    util::run_alt(parse, part1, part2);
}

#[cfg(test)]
#[test]
fn test() {
    for (pkt, sum) in [
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ] {
        assert_eq!(part1(&parse(pkt)), sum);
    }

    for (pkt, val) in [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ] {
        assert_eq!(part2(&parse(pkt)), val);
    }
}
