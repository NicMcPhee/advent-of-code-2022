#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use nom::{
    bytes::complete::tag, character::complete::char, combinator::map, multi::separated_list0,
    sequence::separated_pair, IResult,
};
use std::{collections::HashSet, fmt::Display, fs, ops::RangeInclusive};

use anyhow::{Context, Result};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

struct Cave {
    occupied: HashSet<Point>,
    left_edge: i32,
    right_edge: i32,
    top_edge: i32,    // Smallest y value
    bottom_edge: i32, // Largest y value
}

impl Default for Cave {
    fn default() -> Self {
        Self {
            occupied: HashSet::default(),
            // We know the point (500, 0) is where the sand comes from, so we'll use that as
            // the initial "edge".
            left_edge: 500,
            right_edge: 500,
            top_edge: 0,
            bottom_edge: 0,
        }
    }
}

impl Cave {
    fn add_path(&mut self, path: &Path) {
        for (p, q) in path.points.iter().tuple_windows() {
            self.add_segment(p, q);
        }
    }

    fn add_segment(&mut self, p: &Point, q: &Point) {
        const fn range(l1: i32, l2: i32) -> RangeInclusive<i32> {
            if l1 <= l2 {
                l1..=l2
            } else {
                l2..=l1
            }
        }

        let y_range = range(p.y, q.y);
        for x in range(p.x, q.x) {
            for y in y_range.clone() {
                self.insert(Point { x, y });
            }
        }
    }

    fn insert(&mut self, p: Point) {
        self.left_edge = self.left_edge.min(p.x);
        self.right_edge = self.right_edge.max(p.x);
        self.top_edge = self.top_edge.min(p.y);
        self.bottom_edge = self.bottom_edge.max(p.y);
        self.occupied.insert(p);
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.top_edge..=self.bottom_edge {
            for col in self.left_edge..=self.right_edge {
                if self.occupied.contains(&Point { x: col, y: row }) {
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

static INPUT_FILE: &str = "../inputs/day_14_test.input";

fn point(s: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            nom::character::complete::i32,
            char(','),
            nom::character::complete::i32,
        ),
        Point::new,
    )(s)
}

fn path(s: &str) -> IResult<&str, Path> {
    map(separated_list0(tag(" -> "), point), Path::new)(s)
}

fn parse_path(s: &str) -> anyhow::Result<Path> {
    let (_, p) = path(s).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
    Ok(p)
}

fn main() -> anyhow::Result<()> {
    let paths: Vec<Path> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(parse_path)
        .collect::<Result<_>>()?;

    println!("Our paths are {paths:?}");

    let mut cave = Cave::default();

    for path in paths {
        cave.add_path(&path);
    }

    println!("Our starting cave is\n{cave}");

    Ok(())
}
