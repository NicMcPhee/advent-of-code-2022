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

const FACE_SIZE: usize = 50;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Position {
    const fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }

    // Faces are 50x50.
    // Face 1 is first 50 rows, last 50 cols.
    // Face 2 is first 50 rows, middle 50 cols.
    // Face 3 is second 50 rows, second 50 cols.
    // Face 4 is third 50 rows, second 50 cols.
    // Face 5 is third 50 rows, first 50 cols.
    // Face 6 is fourth 50 rows, first 50 cols.
    fn to_face_position(self) -> FacePosition {
        let face = Face::from_position(&self);
        FacePosition::new(
            self.row % FACE_SIZE,
            self.col % FACE_SIZE,
            face,
            self.direction,
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Face {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Face {
    fn from_position(position: &Position) -> Self {
        match (position.row / 50, position.col / 50) {
            (0, 2) => Face::One,   // Face 1
            (0, 1) => Face::Two,   // Face 2
            (1, 1) => Face::Three, // Face 3
            (2, 1) => Face::Four,  // Face 4
            (2, 0) => Face::Five,  // Face 5
            (3, 0) => Face::Six,   // Face 6
            _ => panic!("Illegal position {position:?}"),
        }
    }

    fn offset(&self) -> (usize, usize) {
        match self {
            Face::One => (0, 2),
            Face::Two => (0, 1),
            Face::Three => (1, 1),
            Face::Four => (2, 1),
            Face::Five => (2, 0),
            Face::Six => (3, 0),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FacePosition {
    row: usize,
    col: usize,
    face: Face,
    direction: Direction,
}

impl FacePosition {
    const fn new(row: usize, col: usize, face: Face, direction: Direction) -> Self {
        Self {
            row,
            col,
            face,
            direction,
    }
    }

    fn forward_one(&self) -> Self {
        // And now we implement a version of MizardX@Twitch's approach!

        // ...

        todo!()
    }

    const fn to_position(&self) -> Position {
        let (row_offset, col_offset) = self.face.offset();
        Position {
            row: self.row + FACE_SIZE * row_offset,
            col: self.col + FACE_SIZE * col_offset,
            direction: self.direction,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Array2<Tile>,
    max_col: usize,
    max_row: usize,
}

impl Map {
    fn empty(num_columns: usize) -> Self {
        Self {
            tiles: Array::from_elem((0, num_columns), Tile::Space),
            max_col: num_columns,
            max_row: 0,
        }
    }

    fn add_row(&mut self, row: &[Tile]) -> anyhow::Result<()> {
        let num_spaces = self.tiles.ncols().checked_sub(row.len()).with_context(|| {
            format!(
                "Num of columns in map was {} and length of row was {}.",
                self.tiles.ncols(),
                row.len()
            )
        })?;
        let padding_spaces = repeat_n(Tile::Space, num_spaces).collect::<Vec<_>>();
        let padded_row = concatenate![Axis(0), row, padding_spaces];
        self.tiles.push_row(padded_row.view())?;
        self.max_row += 1;
        Ok(())
    }

    fn get_by_position(&self, position: Position) -> Tile {
        // `new_position` should always be a legal position on the map, so `get_by_position` should always succeed.
        #[allow(clippy::unwrap_used)]
        // println!("Position is {position:?}.");
        *self.tiles.get((position.row, position.col)).unwrap()
    }

    fn forward_one(position: &Position) -> Position {
        position.to_face_position().forward_one().to_position()
    }

    fn forward(&self, mut position: Position, direction: Direction, num_steps: u32) -> Position {
        for _ in 0..num_steps {
            let new_position = Self::forward_one(&position);
            let tile = self.get_by_position(new_position);
            // println!("New position is {new_position:?} and tile is {tile:?}.");
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
        match tile {
            Tile::Space => self.wrap(new_position, direction),
            Tile::Open => Some(new_position),
            Tile::Wall => None,
        }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
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

#[derive(Debug)]
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
            position: Position::new(0, col, Direction::Right),
            direction: Direction::Right,
        }
    }

    fn act(self, mv: &Action, map: &Map) -> Self {
        // println!("Taking action {mv:?}.");
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

    const fn row(&self) -> usize {
        self.position.row + 1
    }

    const fn col(&self) -> usize {
        self.position.col + 1
    }

    const fn facing(&self) -> usize {
        self.direction as usize
    }

    const fn password(&self) -> usize {
        1000 * self.row() + 4 * self.col() + self.facing()
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

static INPUT_FILE: &str = "../inputs/day_22.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (map, actions) = parse_file(&file)?;

    println!("{map}");

    println!("{actions:?}");

    let you = You::new(&map);

    let you = actions.moves.iter().fold(you, |you, action| {
        // println!("{you:?}");
        you.act(action, &map)
    });

    let password = you.password();

    println!("The value of `You` is {you:?}.");

    println!("The password is {password}");

    Ok(())
}

#[allow(clippy::items_after_test_module)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(
            FacePosition::new(10, 20, Face::One, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::One, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::One, Direction::Right).forward_one(),
            FacePosition::new(39, 49, Face::Four, Direction::Left)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::One, Direction::Left).forward_one(),
            FacePosition::new(10, 49, Face::Two, Direction::Left)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::One, Direction::Up).forward_one(),
            FacePosition::new(49, 20, Face::Six, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::One, Direction::Down).forward_one(),
            FacePosition::new(20, 49, Face::Three, Direction::Left)
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            FacePosition::new(10, 20, Face::Two, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::Two, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::Two, Direction::Right).forward_one(),
            FacePosition::new(10, 0, Face::One, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::Two, Direction::Left).forward_one(),
            FacePosition::new(39, 0, Face::Five, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::Two, Direction::Up).forward_one(),
            FacePosition::new(20, 0, Face::Six, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::Two, Direction::Down).forward_one(),
            FacePosition::new(0, 20, Face::Three, Direction::Down)
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            FacePosition::new(10, 20, Face::Three, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::Three, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::Three, Direction::Right).forward_one(),
            FacePosition::new(49, 10, Face::One, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::Three, Direction::Left).forward_one(),
            FacePosition::new(0, 10, Face::Five, Direction::Down)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::Three, Direction::Up).forward_one(),
            FacePosition::new(49, 20, Face::Two, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::Three, Direction::Down).forward_one(),
            FacePosition::new(0, 49, Face::Four, Direction::Down)
        );
    }

    #[test]
    fn four() {
        assert_eq!(
            FacePosition::new(10, 20, Face::Four, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::Four, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::Four, Direction::Right).forward_one(),
            FacePosition::new(39, 49, Face::One, Direction::Left)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::Four, Direction::Left).forward_one(),
            FacePosition::new(10, 49, Face::Five, Direction::Left)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::Four, Direction::Up).forward_one(),
            FacePosition::new(49, 20, Face::Three, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::Four, Direction::Down).forward_one(),
            FacePosition::new(20, 49, Face::Six, Direction::Left)
        );
    }

    #[test]
    fn five() {
        assert_eq!(
            FacePosition::new(10, 20, Face::Five, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::Five, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::Five, Direction::Right).forward_one(),
            FacePosition::new(10, 0, Face::Four, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::Five, Direction::Left).forward_one(),
            FacePosition::new(39, 0, Face::Two, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::Five, Direction::Up).forward_one(),
            FacePosition::new(20, 0, Face::Three, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::Five, Direction::Down).forward_one(),
            FacePosition::new(0, 20, Face::Six, Direction::Down)
        );
    }

    #[test]
    fn six() {
        assert_eq!(
            FacePosition::new(10, 20, Face::Six, Direction::Right).forward_one(),
            FacePosition::new(10, 21, Face::Six, Direction::Right)
        );
        assert_eq!(
            FacePosition::new(10, 49, Face::Six, Direction::Right).forward_one(),
            FacePosition::new(49, 10, Face::Four, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(10, 0, Face::Six, Direction::Left).forward_one(),
            FacePosition::new(0, 10, Face::Two, Direction::Down)
        );
        assert_eq!(
            FacePosition::new(0, 20, Face::Six, Direction::Up).forward_one(),
            FacePosition::new(49, 20, Face::Five, Direction::Up)
        );
        assert_eq!(
            FacePosition::new(49, 20, Face::Six, Direction::Down).forward_one(),
            FacePosition::new(0, 20, Face::One, Direction::Down)
        );
    }
}
