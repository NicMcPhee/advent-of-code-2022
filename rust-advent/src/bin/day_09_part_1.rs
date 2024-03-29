#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context, Result};
use std::{
    collections::HashSet,
    fs::{self},
    str::FromStr,
};

static INPUT_FILE: &str = "../inputs/day_09.input";

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => bail!("Unknown direction string '{s}'"),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    count: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_ascii_whitespace();
        let direction: Direction = parts
            .next()
            .with_context(|| "Line '{line}' didn't start with a direction")?
            .parse()?;
        let count: usize = parts
            .next()
            .with_context(|| "Line '{line}' didn't have a count component")?
            .parse()?;
        Ok(Self { direction, count })
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn go(&mut self, d: Direction) {
        match d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    const fn dist(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

type Visited = HashSet<Position>;

#[derive(Default)]
struct BridgeState {
    head: Position,
    tail: Position,
    visited: Visited,
}

impl BridgeState {
    fn move_head(&mut self, d: Direction) {
        self.head.go(d);
    }

    fn update_tail(&mut self) {
        match self.head.dist(&self.tail) {
            Position { x: -2, y: _ } => {
                self.tail.x -= 1;
                self.tail.y = self.head.y;
            }
            Position { x: 2, y: _ } => {
                self.tail.x += 1;
                self.tail.y = self.head.y;
            }
            Position { x: _, y: -2 } => {
                self.tail.x = self.head.x;
                self.tail.y -= 1;
            }
            Position { x: _, y: 2 } => {
                self.tail.x = self.head.x;
                self.tail.y += 1;
            }
            _ =>
                /* Do nothing, the tail doesn't have to move */
                {}
        }
    }

    fn process_direction(&mut self, d: Direction) {
        self.move_head(d);
        self.update_tail();
        self.visited.insert(self.tail);
    }

    fn process_move(&mut self, m: &Move) {
        (0..m.count).for_each(|_| self.process_direction(m.direction));
    }

    fn process_moves(&mut self, moves: &[Move]) {
        moves.iter().for_each(|m| self.process_move(m));
    }
}

fn main() -> Result<()> {
    let moves = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Move>>>()?;

    let mut state = BridgeState::default();
    state.process_moves(&moves);
    let num_visited = state.visited.len();

    println!("The number of visited positions was {num_visited}");

    Ok(())
}
