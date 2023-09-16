#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use std::fmt::Display;
use std::ops::{Add, Deref};
use std::{collections::HashMap, fs};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(
        self,
        Self {
            row: delta_row,
            col: delta_col,
        }: Self,
    ) -> Self::Output {
        Self {
            row: self.row + delta_row,
            col: self.col + delta_col,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "^"),
            Self::South => write!(f, "v"),
            Self::West => write!(f, "<"),
            Self::East => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    blizzards: HashMap<Pos, Vec<Blizzard>>,
    num_rows: usize,
    num_cols: usize,
    start: Pos,
    finish: Pos,
}

// TODO: Revisit this without the map, but keeping just a vector of Blizzards, where a
//   knows its position and direction. MizardX@Twitch suggested this, and I'd thought
//   about it a little as well.

impl Map {
    fn occupied(&self, position: &Pos) -> bool {
        self.blizzards.contains_key(position)
    }

    fn blizzards_at(&self, position: &Pos) -> &[Blizzard] {
        self.blizzards
            .get(position)
            .map_or_else(|| &[], Deref::deref)
    }
}

#[allow(clippy::expect_used)]
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0usize..self.num_rows {
            for col in 0..self.num_cols {
                let pos = Pos::new(row, col);
                if pos != self.start
                    && pos != self.finish
                    && (row == 0
                        || col == 0
                        || row == self.num_rows - 1
                        || col == self.num_cols - 1)
                {
                    write!(f, "#")?;
                    continue;
                }
                match self.blizzards_at(&pos) {
                    [] => write!(f, ".")?,
                    [b] => write!(f, "{}", b.direction)?,
                    blizzards => write!(f, "{}", &blizzards.len())?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn process_line(blizzards: &mut HashMap<Pos, Vec<Blizzard>>, row: usize, line: &str) {
    for (col, c) in line.chars().enumerate() {
        match c {
            '#' | '.' => {}
            _ => {
                let direction = match c {
                    '<' => Direction::West,
                    '>' => Direction::East,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => unreachable!("We received a character {c} that shouldn't have happened"),
                };
                blizzards
                    .entry(Pos::new(row, col))
                    .or_default()
                    .push(Blizzard { direction });
            }
        }
    }
}

fn parse_map(file_contents: &str) -> Map {
    let mut blizzards: HashMap<Pos, Vec<Blizzard>> = HashMap::new();
    let mut num_rows = usize::MIN;
    let mut num_cols = 0;
    for (row, line) in file_contents.lines().enumerate() {
        process_line(&mut blizzards, row, line);
        num_rows = row;
        num_cols = line.len();
    }
    Map {
        blizzards,
        num_rows: num_rows + 1,
        num_cols,
        start: Pos::new(0, 1),
        finish: Pos::new(num_rows, num_cols - 2),
    }
}

static INPUT_FILE: &str = "../inputs/day_24_test.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let map = parse_map(&file);
    println!("Initial map: \n{map}");
    println!("{}, {}", map.num_rows, map.num_cols);
    println!("{:?}, {:?}", map.start, map.finish);

    Ok(())
}
