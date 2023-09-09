#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use itertools::Itertools;
use std::fmt::Display;
use std::ops::{Add, Not};
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
            Direction::North => self + (-1, 0),
            Direction::South => self + (1, 0),
            Direction::West => self + (0, -1),
            Direction::East => self + (0, 1),
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
        direction
            .offsets()
            .into_iter()
            .any(|offset| board.occupied(self.position + offset))
            .not()
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
        self.elves.contains(&Elf { position })
    }

    #[allow(clippy::unwrap_used)]
    fn row_bounds(&self) -> (isize, isize) {
        self.elves
            .iter()
            .map(|elf| elf.position.row)
            .minmax()
            .into_option()
            .unwrap()
    }

    #[allow(clippy::unwrap_used)]
    fn col_bounds(&self) -> (isize, isize) {
        self.elves
            .iter()
            .map(|elf| elf.position.col)
            .minmax()
            .into_option()
            .unwrap()
    }

    #[allow(clippy::unwrap_used)]
    fn empty_ground_tiles(&self) -> usize {
        let (min_row, max_row) = self.row_bounds();
        let (min_col, max_col) = self.col_bounds();
        let area = usize::try_from((max_row - min_row + 1) * (max_col - min_col + 1)).unwrap();
        area - self.elves.len()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_row, max_row) = self.row_bounds();
        let (min_col, max_col) = self.col_bounds();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if self.occupied(Position { row, col }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
        // .map(|(col, c)| Elf::new(row, isize::try_from(col).unwrap()))
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

    /// Generate the three offsets in the specified direction, i.e.,
    /// NW, N, and NE if the direction is North.
    const fn offsets(self) -> [(isize, isize); 3] {
        match self {
            Self::North => [(-1, -1), (-1, 0), (-1, 1)],
            Self::South => [(1, -1), (1, 0), (1, 1)],
            Self::West => [(-1, -1), (0, -1), (1, -1)],
            Self::East => [(-1, 1), (0, 1), (1, 1)],
        }
    }
}

fn one_round(board: &Board, directions: &Vec<Direction>) -> Board {
    // println!("{directions:?}");
    let mut proposals: HashMap<Position, Vec<Elf>> = HashMap::new();
    for elf in &board.elves {
        // println!("Elf: {elf:?}");
        let proposed_move = elf.propose_move(board, directions);
        // println!("Proposed move: {proposed_move:?}");
        let entry = proposals.entry(proposed_move);
        entry.or_default().push(*elf);
    }
    // println!("Proposals: {proposals:?}");
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

static INPUT_FILE: &str = "../inputs/day_23.input";

fn main() -> anyhow::Result<()> {
    const NUM_ROUNDS: usize = 10;

    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let board = parse_map(&file);
    println!("Initial board: \n{board}");

    // let board = one_round(&board, &Direction::cycle().take(4).collect());
    // println!("After one round: \n{board}");

    // let board = one_round(&board, &Direction::cycle().skip(1).take(4).collect());
    // println!("After two_rounds: \n{board}");

    let final_elves = disperse_elves(board, NUM_ROUNDS);

    println!("After dispersal: \n{final_elves}");

    let result = final_elves.empty_ground_tiles();

    println!("The result = {result}.");

    Ok(())
}
