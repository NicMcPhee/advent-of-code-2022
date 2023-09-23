#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use pathfinding::directed::astar::astar;
use std::fmt::Display;
use std::ops::Add;
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

#[derive(Eq, PartialEq, Hash, Clone)]
struct Node {
    pos: Pos,
    time: usize,
}

impl Node {
    const fn new(pos: Pos, time: usize) -> Self {
        Self { pos, time }
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

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    blizzards: HashMap<Pos, Blizzard>,
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

    fn blizzard_at(&self, position: &Pos) -> Option<Blizzard> {
        self.blizzards.get(position).copied()
    }

    // The plan is to use MizardX@Twitch's idea of wrapping, so we leave
    // the map unchanged, and just move the blizzards `t` time steps
    // in their direction and see if they get in the way.
    //
    // We know where we are, so we know which positions problematic
    // blizzards could be in, and we can reverse time to find out where
    // those blizzards would have needed to be in the initial map, and
    // then just look them up.
    fn successors(&self, node: &Node) -> Vec<(Node, usize)> {
        todo!()
    }

    fn finished(&self, node: &Node) -> bool {
        node.pos == self.finish
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
                match self.blizzard_at(&pos) {
                    None => write!(f, ".")?,
                    Some(blizzard) => write!(f, "{}", blizzard.direction)?,
                    // [] => write!(f, ".")?,
                    // [b] => write!(f, "{}", b.direction)?,
                    // blizzards => write!(f, "{}", &blizzards.len())?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn process_line(blizzards: &mut HashMap<Pos, Blizzard>, row: usize, line: &str) {
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
                blizzards.insert(Pos::new(row, col), Blizzard { direction });
            }
        }
    }
}

fn parse_map(file_contents: &str) -> Map {
    let mut blizzards: HashMap<Pos, Blizzard> = HashMap::new();
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

    let Some((_, num_minutes)) = astar(
        &Node::new(map.start, 0),
        |node| map.successors(node),
        |node| map.heuristic(node),
        |node| map.finished(node),
    ) else {
        unreachable!("Dijkstra should have returned a successful path.")
    };

    println!("The number of minutes was {num_minutes}.");

    Ok(())
}
