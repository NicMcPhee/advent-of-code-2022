#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use std::ops::Add;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::North => self + (0, -1),
            Direction::South => self + (0, 1),
            Direction::West => self + (-1, 0),
            Direction::East => self + (1, 0),
        }
    }
}

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, (delta_row, delta_col): (isize, isize)) -> Self::Output {
        Self {
            row: self.row + delta_row,
            col: self.col + delta_col,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Elf {
    position: Position,
}

impl Elf {
    const fn new(row: isize, col: isize) -> Self {
        Self {
            position: Position { row, col },
        }
    }

    /// Return `true` if there are no elves in any of the 8 adjacent positions.
    fn isolated(&self, board: &Board) -> bool {
        for row_delta in -1..=1 {
            for col_delta in -1..=1 {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                };
                if board.occupied(self.position + (row_delta, col_delta)) {
                    return false;
                }
            }
        }
        true
    }

    fn can_move(&self, board: &Board, direction: Direction) -> bool {
        todo!()
    }

    // If we don't propose moving, we return our current position as the proposal
    // as a way of saying we need to stay put. No other elf will propose moving
    // into this position, so that should be safe.
    fn propose_move(&self, board: &Board, directions: &Vec<Direction>) -> Position {
        if self.isolated(board) {
            return self.position;
        }
        for direction in directions {
            if self.can_move(board, *direction) {
                return self.position.shift(*direction);
            }
        }
        self.position
    }
}

#[derive(Debug)]
struct Board {
    elves: HashSet<Elf>,
}

impl Board {
    fn occupied(&self, position: Position) -> bool {
        todo!()
    }
}

fn parse_map_row(row: isize, s: &str) -> Vec<Elf> {
    #[allow(clippy::unwrap_used)]
    s.chars()
        .enumerate()
        .filter_map(|(col, c)| match c {
            '#' => Some(Elf::new(row, isize::try_from(col).unwrap())),
            _ => None,
        })
        // .filter(|(col, c)| *c == '#')
        // .map(|(col, c)| Elf::new(row, col))
        .collect()
}

fn parse_map(file_contents: &str) -> Board {
    #[allow(clippy::unwrap_used)]
    let elves: HashSet<Elf> = file_contents
        .lines()
        .enumerate()
        .flat_map(|(row, row_contents)| parse_map_row(isize::try_from(row).unwrap(), row_contents))
        .collect();
    Board { elves }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn cycle() -> impl Iterator<Item = Self> {
        [Self::North, Self::South, Self::West, Self::East]
            .into_iter()
            .cycle()
    }
}

fn one_round(board: &Board, directions: &Vec<Direction>) -> Board {
    let mut proposals: HashMap<Position, Vec<Elf>> = HashMap::new();
    for elf in &board.elves {
        let proposed_move = elf.propose_move(board, directions);
        let entry = proposals.entry(proposed_move);
        entry.or_default().push(*elf);
    }
    Board {
        elves: proposals
            .into_iter()
            .flat_map(|(position, elves)| {
                if elves.len() == 1 {
                    vec![Elf { position }]
                } else {
                    elves
                }
            })
            .collect(),
    }
}

fn disperse_elves(mut board: Board, num_rounds: usize) -> Board {
    let mut directions_cycle = Direction::cycle();

    for _ in 0..num_rounds {
        let directions = directions_cycle.by_ref().take(4).collect();
        directions_cycle.next();
        board = one_round(&board, &directions);
    }
    board
}

fn empty_ground_tiles(board: &Board) -> usize {
    todo!()
}

static INPUT_FILE: &str = "../inputs/day_23_small_test.input";

fn main() -> anyhow::Result<()> {
    const NUM_ROUNDS: usize = 10;

    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let board = parse_map(&file);
    println!("{board:?}");

    let board = one_round(&board, &Direction::cycle().collect());
    println!("{board:?}");

    let board = one_round(&board, &Direction::cycle().skip(1).collect());
    println!("{board:?}");

    let final_elves = disperse_elves(board, NUM_ROUNDS);

    let result = empty_ground_tiles(&final_elves);

    println!("The result = {result}.");

    Ok(())
}
