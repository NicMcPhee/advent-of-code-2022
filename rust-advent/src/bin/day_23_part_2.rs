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
    fn propose_move(&self, board: &Board, directions: &Vec<Direction>) -> Option<Position> {
        if self.isolated(board) {
            return None;
        }
        for direction in directions {
            if self.can_move(board, *direction) {
                return Some(self.position.shift(*direction));
            }
        }
        None
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

fn one_round(board: &Board, directions: &Vec<Direction>) -> Option<Board> {
    let mut proposals: HashMap<Position, Vec<Elf>> = HashMap::new();
    let mut num_nontrivial_proposals = 0;
    for elf in &board.elves {
        let proposed_move =
            elf.propose_move(board, directions)
                .map_or(elf.position, |proposed_move| {
                    num_nontrivial_proposals += 1;
                    proposed_move
                });
        proposals.entry(proposed_move).or_default().push(*elf);
    }
    if num_nontrivial_proposals == 0 {
        return None;
    }
    Some(Board {
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
    })
}

fn disperse_elves(mut board: Board) -> usize {
    // TODO: Instead of cycling through the whole list of directions, we could cycle through
    //   a list of the 4 possible arrays of 4 directions: [NSWE], [SWEN], [WENS], and [ENSW].
    //   We could use `.windows()` on the iterator resulting from `.cycle()` to get an "infinite"
    //   iterator yielding iterators over 4 elements.
    let mut directions_cycle = Direction::cycle();

    let mut num_rounds = 1;
    while let Some(new_board) = one_round(&board, &directions_cycle.by_ref().take(4).collect()) {
        board = new_board;
        directions_cycle.next();
        num_rounds += 1;
    }
    num_rounds
}

static INPUT_FILE: &str = "../inputs/day_23.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let board = parse_map(&file);
    println!("Initial board: \n{board}");

    let num_rounds = disperse_elves(board);

    println!("The result = {num_rounds}.");

    Ok(())
}
