#![cfg_attr(test, feature(test))]

use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Copy, Clone)]
struct Board([[u8; 5]; 5]);

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let n = self.0[y][x];
                if n <= 9 {
                    write!(f, " ")?;
                }
                if x > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", n)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct BoardState([[bool; 5]; 5]);

impl BoardState {
    fn has_won(&self) -> bool {
        for i in 0..5 {
            if (0..5).all(|j| self.0[i][j]) || (0..5).all(|j| self.0[j][i]) {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, board: &Board, n: u8) {
        for y in 0..5 {
            for x in 0..5 {
                if board.0[y][x] == n {
                    self.0[y][x] = true;
                    return;
                }
            }
        }
    }

    fn sum_of_unmarked(&self, board: &Board) -> u32 {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.0[y][x] {
                    sum += board.0[y][x] as u32;
                }
            }
        }
        sum
    }
}

fn parse_input(data: &str) -> Option<(Vec<u8>, Vec<Board>)> {
    let calls = data
        .lines()
        .next()?
        .split(",")
        .map(util::parse_unwrap::<u8>)
        .collect::<Vec<u8>>();

    let board_lines = data.lines().skip(1).collect::<Vec<_>>();

    let mut boards = Vec::new();
    for i in (1..board_lines.len() - 4).step_by(6) {
        let mut board = Board::default();
        for y in 0..5 {
            let line = &board_lines[i + y];
            for (x, n) in line.split_ascii_whitespace().enumerate() {
                board.0[y][x] = n.parse().unwrap();
            }
        }
        boards.push(board);
    }

    Some((calls, boards))
}

fn part1((calls, boards): &(Vec<u8>, Vec<Board>)) -> u32 {
    let mut boards = boards.clone();
    let mut states = vec![BoardState::default(); boards.len()];
    for &call in calls {
        for (state, board) in states.iter_mut().zip(&mut boards) {
            state.mark(board, call);
            if state.has_won() {
                return state.sum_of_unmarked(board) * call as u32;
            }
        }
    }
    panic!("no winner");
}

fn part2((calls, boards): &(Vec<u8>, Vec<Board>)) -> u32 {
    let mut boards = boards.clone();
    let mut states = vec![BoardState::default(); boards.len()];
    for &call in calls {
        for (state, board) in states.iter_mut().zip(&mut boards) {
            state.mark(board, call);
        }
        if states.len() == 1 && states[0].has_won() {
            return states[0].sum_of_unmarked(&boards[0]) * call as u32;
        }
        for i in (0..states.len()).rev() {
            if states[i].has_won() {
                states.remove(i);
                boards.remove(i);
            }
        }
    }
    panic!("no last-place");
}

util::register_alt!(|s| parse_input(s).unwrap(), part1, part2);
