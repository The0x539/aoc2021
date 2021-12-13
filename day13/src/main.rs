use std::collections::BTreeSet;

#[derive(Default)]
struct Input {
    dots: Vec<(i32, i32)>,
    folds: Vec<(bool, i32)>,
}

type Output = usize;

fn parse(s: &str) -> Input {
    let mut inp = Input::default();
    let mut lines = s.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let (l, r) = line.trim().split_once(",").unwrap();
        inp.dots.push((l.parse().unwrap(), r.parse().unwrap()));
    }
    for line in lines {
        let i = line.find('=').unwrap();
        let is_y = line.as_bytes()[i - 1] == b'y';
        let coord = line[i + 1..].trim().parse().unwrap();
        inp.folds.push((is_y, coord));
    }
    inp
}

#[derive(Default)]
struct Grid(BTreeSet<(i32, i32)>);

impl Grid {
    fn from_input(inp: &Input) -> Self {
        let mut g = Self::default();
        for &(x, y) in &inp.dots {
            g.0.insert((x, y));
        }
        g
    }

    fn fold(&self, along_y: bool, coord: i32) -> Self {
        let mut g = Self::default();
        for &(mut x, mut y) in &self.0 {
            if along_y {
                if y > coord {
                    y = coord - (y - coord);
                }
            } else {
                if x > coord {
                    x = coord - (x - coord);
                }
            }
            g.0.insert((x, y));
        }
        g
    }
}

fn part1(inp: &Input) -> Output {
    let grid = Grid::from_input(inp);
    let &(axis, coord) = dbg!(&inp.folds[0]);
    let new_grid = grid.fold(axis, coord);

    new_grid.0.len()
}

fn part2(inp: &Input) -> Output {
    let mut grid = Grid::from_input(inp);
    for &(axis, coord) in &inp.folds {
        grid = grid.fold(axis, coord);
    }
    let x_min = grid.0.iter().map(|(x, _)| *x).min().unwrap();
    let y_min = grid.0.iter().map(|(_, y)| *y).min().unwrap();
    let x_max = grid.0.iter().map(|(x, _)| *x).max().unwrap();
    let y_max = grid.0.iter().map(|(_, y)| *y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = if grid.0.contains(&(x, y)) { '#' } else { ' ' };
            print!("{}", c);
        }
        println!();
    }
    0
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
#[test]
fn test() {
    let (a, b) = util::parse_output::<usize>();
    let input = std::fs::read_to_string("test.txt").unwrap();
    let input = parse(&input);
    assert_eq!(a, part1(&input));
    assert_eq!(b, part2(&input));
}
