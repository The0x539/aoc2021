#![cfg_attr(test, feature(test))]

fn parse_bits(bits: &[bool]) -> u32 {
    let mut n = 0;
    for bit in bits {
        n <<= 1;
        n |= *bit as u32;
    }
    n
}

fn parse(s: &str) -> Vec<bool> {
    s.bytes().map(|b| b == b'1').collect()
}

fn part1(input: &[Vec<bool>]) -> u32 {
    let mut gamma = vec![];
    let mut epsilon = vec![];
    for i in 0..input[0].len() {
        let mut count = 0;
        for num in input {
            count += num[i] as usize;
        }
        let gamma_bit = count >= input.len() / 2;
        let epsilon_bit = !gamma_bit;
        gamma.push(gamma_bit);
        epsilon.push(epsilon_bit);
    }
    let gamma_n = parse_bits(&gamma);
    let epsilon_n = parse_bits(&epsilon);
    gamma_n * epsilon_n
}

fn part2(input: &[Vec<bool>]) -> u32 {
    let len = input[0].len();

    let most_common_bit_in_pos = |i: usize, input: &[Vec<bool>]| {
        let min_len = input.len() / 2 + input.len() % 2;
        input.iter().map(|v| v[i] as usize).sum::<usize>() >= min_len
    };

    let least_common_bit_in_pos = |i: usize, input: &[Vec<bool>]| {
        let max_len = input.len() / 2 + input.len() % 2;
        input.iter().map(|v| v[i] as usize).sum::<usize>() < max_len
    };

    let mut o2_rating = input.to_vec();
    let mut co2_rating = input.to_vec();
    for i in 0..len {
        let mcb = most_common_bit_in_pos(i, &o2_rating);
        o2_rating.retain(|num| num[i] == mcb);
        if o2_rating.len() == 1 {
            break;
        }
    }
    for i in 0..len {
        let lcb = least_common_bit_in_pos(i, &co2_rating);
        co2_rating.retain(|num| num[i] == lcb);
        if co2_rating.len() == 1 {
            break;
        }
    }
    assert_eq!(o2_rating.len(), 1);
    assert_eq!(co2_rating.len(), 1);
    parse_bits(&co2_rating[0]) * parse_bits(&o2_rating[0])
}

util::register!(parse, part1, part2);
