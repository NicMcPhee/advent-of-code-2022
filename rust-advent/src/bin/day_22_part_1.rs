#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use itertools::repeat_n;
use ndarray::{concatenate, Array, Array2, Axis};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::{all_consuming, map},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{fmt::Display, fs};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Space,
    Open,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, " "),
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Array2<Tile>,
}

impl Map {
    fn empty(num_columns: usize) -> Self {
        Self {
            tiles: Array::from_elem((0, num_columns), Tile::Space),
        }
    }

    fn add_row(&mut self, row: &[Tile]) -> anyhow::Result<()> {
        let num_spaces = self.tiles.ncols() - row.len();
        let padding_spaces = repeat_n(Tile::Space, num_spaces).collect::<Vec<_>>();
        let padded_row = concatenate![Axis(0), row, padding_spaces];
        self.tiles.push_row(padded_row.view())?;
        Ok(())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.rows() {
            for tile in row.iter() {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_map_row(s: &str) -> IResult<&str, Vec<Tile>> {
    many1(alt((
        map(tag(" "), |_| Tile::Space),
        map(tag("."), |_| Tile::Open),
        map(tag("#"), |_| Tile::Wall),
    )))(s)
}

fn parse_map(s: &str) -> IResult<&str, Map> {
    let (rest, rows) = separated_list1(newline, parse_map_row)(s)?;
    let max_width = rows.iter().map(std::vec::Vec::len).max().unwrap();
    let mut map = Map::empty(max_width);

    for line in &rows {
        map.add_row(line).unwrap();
    }
    Ok((rest, map))
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Forward(i32),
}

#[derive(Debug)]
struct Directions {
    moves: Vec<Move>,
}

fn parse_directions(s: &str) -> IResult<&str, Directions> {
    let (rest, moves) = many1(alt((
        map(i32, Move::Forward),
        map(tag("L"), |_| Move::Left),
        map(tag("R"), |_| Move::Right),
    )))(s)?;
    Ok((rest, Directions { moves }))
}

fn parse_file(contents: &str) -> anyhow::Result<(Map, Directions)> {
    let (_, (map, directions)) = all_consuming(terminated(
        separated_pair(parse_map, many1(newline), parse_directions),
        many0(newline),
    ))(contents)
    .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)
    .context("Failed to parse the input file.")?;
    Ok((map, directions))
}

static INPUT_FILE: &str = "../inputs/day_22_test.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (map, directions) = parse_file(&file)?;

    println!("{map}");

    println!("{directions:?}");

    //     .map(get_monkey)
    //     .collect::<anyhow::Result<HashMap<MonkeyName, Monkey>>>()?;
    // let mut monkeys = Monkeys { monkeys };

    // println!("{monkeys:?}");

    // let Monkey::Expression(_, left, right) =
    //     monkeys.monkeys.get(&MonkeyName::new("root")).context("Failed to get the root monkey")?.clone()
    // else {
    //     panic!("The root monkey didn't map to an expression")
    // };

    // let left_value = monkeys.get_value(&left)?;
    // let right_value = monkeys.get_value(&right)?;
    // println!("Left = {left_value:?}");
    // println!("Right = {right_value:?}");

    // let difference = left_value - right_value;

    // println!("Difference = {difference:?}");

    // // difference = a + bx
    // // We need a + bx = 0
    // //   == bx = -a
    // //   == x = -a/b

    // let result = -difference.constant / difference.coefficient;

    // println!("Result = {result:?}");

    // Ok(())

    todo!()
}
