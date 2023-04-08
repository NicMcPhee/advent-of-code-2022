#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{sequence::delimited, IResult};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new((x, y): (i32, i32)) -> Self {
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
    let (_, p) = path(s).map_err(|e| e.to_owned())?;
    Ok(p)
}

fn main() -> anyhow::Result<()> {
    let lines: Vec<Path> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(|line| parse_path(line))
        .collect::<Result<_>>()?;

    println!("Our paths are {lines:?}");

    Ok(())
}
