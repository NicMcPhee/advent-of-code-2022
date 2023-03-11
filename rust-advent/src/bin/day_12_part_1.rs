#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    str::FromStr,
};

use anyhow::{Context, Result};

#[derive(Debug)]
enum Height {
    Start,
    End,
    Height(u8),
}

impl Height {
    fn new(c: char) -> Self {
        match c {
            'S' => Self::Start, // Start has height 0
            'E' => Self::End,   // End has height 26
            _ => Self::Height(c as u8 - 'a' as u8),
        }
    }
}

#[derive(Debug, Clone)]
struct Location {
    x: usize,
    y: usize,
    dist: u32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Eq for Location {}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist) 
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Terrain {
    heights: Vec<Vec<Height>>,
    start: Location,
    end: Location,
}

impl FromStr for Terrain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start: Option<Location> = None;
        let mut end: Option<Location> = None;
        let mut heights: Vec<Vec<Height>> = Vec::new();
        for (x, line) in s.lines().enumerate() {
            let mut row_heights: Vec<Height> = Vec::with_capacity(line.len());
            for (y, c) in line.chars().enumerate() {
                let height = Height::new(c);
                match height {
                    Height::Start => start = Some(Location { x, y, dist: u32::MAX }),
                    Height::End => end = Some(Location { x, y, dist: u32::MAX }),
                    _ => { /* Do nothing */}
                };
                row_heights.push(height);
            }
            heights.push(row_heights);
        }
        let start = start.context("We never found the start location")?;
        let end = end.context("We never found the end location")?;
        Ok(Terrain { heights, start, end })
    }
}

impl Terrain {}

static INPUT_FILE: &str = "../inputs/day_12_test.input";

fn main() -> Result<()> {
    let terrain = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .parse::<Terrain>()?;

    println!("{terrain:?}");

    todo!();

    Ok(())
}
