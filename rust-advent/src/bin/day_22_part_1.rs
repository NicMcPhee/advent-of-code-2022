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
    character::complete::{newline, u32},
    combinator::{all_consuming, map},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{fmt::Display, fs};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    // TODO: Pass `Map` as an additional argument, wrap when necessary,
    //   and return `Self` instead of `Option<Self>`.
    fn forward_one(&self, direction: Direction, max_x: usize, max_y: usize) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        match direction {
            Direction::Left => x = x.checked_sub(1).unwrap_or(max_x - 1),
            Direction::Right => x = (x + 1) % max_x,
            Direction::Up => y = y.checked_sub(1).unwrap_or(max_y - 1),
            Direction::Down => y = (y + 1) % max_y,
        };
        Self { x, y }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Array2<Tile>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn empty(num_columns: usize) -> Self {
        Self {
            tiles: Array::from_elem((0, num_columns), Tile::Space),
            max_x: num_columns,
            max_y: 0,
        }
    }

    fn add_row(&mut self, row: &[Tile]) -> anyhow::Result<()> {
        let num_spaces = self.tiles.ncols() - row.len();
        let padding_spaces = repeat_n(Tile::Space, num_spaces).collect::<Vec<_>>();
        let padded_row = concatenate![Axis(0), row, padding_spaces];
        self.tiles.push_row(padded_row.view())?;
        self.max_y += 1;
        Ok(())
    }

    fn get_by_position(&self, position: Position) -> Tile {
        // `new_position` should always be a legal position on the map, so `get_by_position` should always succeed.
        #[allow(clippy::unwrap_used)]
        *self.tiles.get((position.x, position.y)).unwrap()
    }

    fn forward_one(&self, position: &Position, direction: Direction) -> Position {
        position.forward_one(direction, self.max_x, self.max_y)
    }

    fn forward(&self, mut position: Position, direction: Direction, num_steps: u32) -> Position {
        for _ in 0..num_steps {
            let new_position = self.forward_one(&position, direction);
            let tile = self.get_by_position(new_position);
            position = match tile {
                Tile::Space => match self.wrap(position, direction) {
                    Some(new_position) => new_position,
                    None => return position,
                },
                Tile::Open => new_position,
                Tile::Wall => return position,
            }
        }
        position
    }

    // This is called if we've run into a `Tile::Space`, which means we need to keep
    // going in the current direction (wrapping around the map edge is handled by
    // Position::forward_one) until we find a non-space tile. If that tile is a
    // `Tile::Wall`, then we can't wrap in that direction and we return `None`. If
    // it's a `Tile::Open` than that's the `new_position` that we've moved to and
    // we want to return that.
    fn wrap(&self, position: Position, direction: Direction) -> Option<Position> {
        let new_position = self.forward_one(&position, direction);
        let tile = self.get_by_position(new_position);
        if position == new_position {}
        todo!()
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

#[allow(clippy::unwrap_used)]
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
enum Action {
    Left,
    Right,
    Forward(u32),
}

#[derive(Debug)]
struct Actions {
    moves: Vec<Action>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    const fn turn_right(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
}

struct You {
    position: Position,
    direction: Direction,
}

impl You {
    fn new(map: &Map) -> Self {
        let top_row = map.tiles.row(0);
        #[allow(clippy::unwrap_used)]
        let col = top_row.iter().position(|tile| tile == &Tile::Open).unwrap();
        Self {
            position: Position::new(0, col),
            direction: Direction::Right,
        }
    }

    fn act(self, mv: &Action, map: &Map) -> Self {
        match mv {
            Action::Left => self.turn_left(),
            Action::Right => self.turn_right(),
            Action::Forward(num_steps) => self.forward(*num_steps, map),
        }
    }

    const fn turn_left(self) -> Self {
        Self {
            direction: self.direction.turn_left(),
            ..self
        }
    }

    const fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            ..self
        }
    }

    fn forward(self, num_steps: u32, map: &Map) -> Self {
        let position = map.forward(self.position, self.direction, num_steps);
        Self { position, ..self }
    }

    fn password(&self) -> usize {
        todo!()
    }
}

fn parse_directions(s: &str) -> IResult<&str, Actions> {
    let (rest, moves) = many1(alt((
        map(u32, Action::Forward),
        map(tag("L"), |_| Action::Left),
        map(tag("R"), |_| Action::Right),
    )))(s)?;
    Ok((rest, Actions { moves }))
}

fn parse_file(contents: &str) -> anyhow::Result<(Map, Actions)> {
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

    let (map, actions) = parse_file(&file)?;

    println!("{map}");

    println!("{actions:?}");

    let you = You::new(&map);

    let you = actions
        .moves
        .iter()
        .fold(you, |you, action| you.act(action, &map));

    let password = you.password();

    println!("The password is {password}");

    Ok(())
}
