#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context, Result};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::{self},
    str::FromStr,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
enum Height {
    Start,
    End,
    Height(u8),
}

impl Height {
    const fn new(c: char) -> Self {
        match c {
            'S' => Self::Start, // Start has height 0
            'E' => Self::End,   // End has height 26
            _ => Self::Height(c as u8 - b'a'),
        }
    }

    const fn get_height(&self) -> u8 {
        match self {
            Self::Start => 0,
            Self::End => 25,
            Self::Height(h) => *h,
        }
    }
}

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn neighbor(&self, direction: &Direction) -> Option<Self> {
        let mut row = self.row;
        let mut col = self.col;
        match direction {
            Direction::Up => col = col.checked_sub(1)?,
            Direction::Down => col += 1,
            Direction::Left => row = row.checked_sub(1)?,
            Direction::Right => row += 1,
        }
        Some(Self { row, col })
    }
}

#[derive(Debug)]
struct Terrain {
    heights: Vec<Vec<Height>>,
    start: Location,
}

impl FromStr for Terrain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start: Option<Location> = None;
        let mut heights: Vec<Vec<Height>> = Vec::new();
        for (row, line) in s.lines().enumerate() {
            let mut row_heights: Vec<Height> = Vec::with_capacity(line.len());
            for (col, c) in line.chars().enumerate() {
                let height = Height::new(c);
                if matches!(height, Height::Start) {
                    start = Some(Location { row, col });
                }
                row_heights.push(height);
            }
            heights.push(row_heights);
        }
        let start = start.context("We never found the start location")?;
        Ok(Self { heights, start })
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Node {
    dist: u32,
    location: Location,
}

impl Terrain {
    fn get_height(&self, location: &Location) -> Option<u8> {
        self.heights
            .get(location.row)
            .and_then(|row| row.get(location.col))
            .map(Height::get_height)
    }

    fn accessible_locations<'a>(
        &'a self,
        node: &'a Node,
        current_height: u8,
    ) -> impl Iterator<Item = Location> + 'a {
        Direction::iter()
            .filter_map(|direction| node.location.neighbor(&direction))
            .map(|location| (self.get_height(&location), location))
            .filter_map(move |(ht, location)| {
                ht.and_then(|ht| (ht <= current_height + 1).then_some(location))
            })
    }

    fn shortest_path_length(&self) -> Result<u32> {
        // This HashMap maps `Location`s to `u32`, keeping track of the shortest known
        //   paths to given locations.
        let mut best_distance = HashMap::new();

        let mut open_list: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        open_list.push(Reverse(Node {
            location: self.start.clone(),
            dist: 0,
        }));

        while let Some(Reverse(node)) = open_list.pop() {
            let height = &self.heights[node.location.row][node.location.col];

            if matches!(height, Height::End) {
                return Ok(node.dist);
            }

            let bd = best_distance.get(&node.location).copied();
            // println!("Processing {node:?} with height {height:?} and best distance {bd:?}");
            if let Some(dist) = bd {
                if dist <= node.dist {
                    continue;
                }
            }

            best_distance.insert(node.location.clone(), node.dist);
            let accessible = self.accessible_locations(&node, height.get_height());
            // I could replace this `for` loop with a `for_each()` call.
            // That would probably make the most sense if I brought the body
            // of `self.accessible_locations` back to this function, and
            // I'm not sure I'm jazzed about that.
            for location in accessible {
                let new_node = Node {
                    location,
                    dist: node.dist + 1,
                };
                // println!("\tPushing node {new_node:?}");
                open_list.push(Reverse(new_node));
            }
        }

        bail!("We failed to find the end location!")
    }
}

static INPUT_FILE: &str = "../inputs/day_12.input";

fn main() -> Result<()> {
    let terrain = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .parse::<Terrain>()?;

    // println!("{terrain:?}");

    let shortest_path_length = terrain.shortest_path_length()?;

    println!("The shortest path length was {shortest_path_length}.");

    Ok(())
}
