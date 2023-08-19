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
use std::ops::{Add, Sub};
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

// TODO: Possibly refactor the key structures.
// The three structures `Position`, `FacePosition`, and `You` all have a lot of
// overlap and exist as three structures in significant part as historical
// artifacts, especially in going from Part 1 to Part 2 of the problem.
//
// We can probably combine these into two or possibly just one `struct` and
// simplify the structure of the code.
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
            (0, 2) => Self::One,   // Face 1
            (0, 1) => Self::Two,   // Face 2
            (1, 1) => Self::Three, // Face 3
            (2, 1) => Self::Four,  // Face 4
            (2, 0) => Self::Five,  // Face 5
            (3, 0) => Self::Six,   // Face 6
            _ => panic!("Illegal position {position:?}"),
        }
    }

    const fn offset(self) -> (usize, usize) {
        match self {
            Self::One => (0, 2),
            Self::Two => (0, 1),
            Self::Three => (1, 1),
            Self::Four => (2, 1),
            Self::Five => (2, 0),
            Self::Six => (3, 0),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct FacePosition {
    row: usize,
    col: usize,
    face: Face,
    direction: Direction,
}

impl Add<(usize, usize)> for FacePosition {
    type Output = Self;

    fn add(self, (row_delta, col_delta): (usize, usize)) -> Self::Output {
        Self {
            row: self.row + row_delta,
            col: self.col + col_delta,
            ..self
        }
    }
}

impl Sub<(usize, usize)> for FacePosition {
    type Output = Self;

    fn sub(self, (row_delta, col_delta): (usize, usize)) -> Self::Output {
        Self {
            row: self.row - row_delta,
            col: self.col - col_delta,
            ..self
        }
    }
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

    // Copied from code shared by MizardX@Twitch.
    const fn wrap(&self) -> (Face, Direction) {
        match (self.face, self.direction) {
            (Face::One, Direction::Left) => (Face::Two, Direction::Left),
            (Face::One, Direction::Up) => (Face::Six, Direction::Up),
            (Face::One, Direction::Right) => (Face::Four, Direction::Left),
            (Face::One, Direction::Down) => (Face::Three, Direction::Left),
            (Face::Two, Direction::Left) => (Face::Five, Direction::Right),
            (Face::Two, Direction::Up) => (Face::Six, Direction::Right),
            (Face::Two, Direction::Right) => (Face::One, Direction::Right),
            (Face::Two, Direction::Down) => (Face::Three, Direction::Down),
            (Face::Three, Direction::Left) => (Face::Five, Direction::Down),
            (Face::Three, Direction::Up) => (Face::Two, Direction::Up),
            (Face::Three, Direction::Right) => (Face::One, Direction::Up),
            (Face::Three, Direction::Down) => (Face::Four, Direction::Down),
            (Face::Four, Direction::Left) => (Face::Five, Direction::Left),
            (Face::Four, Direction::Up) => (Face::Three, Direction::Up),
            (Face::Four, Direction::Right) => (Face::One, Direction::Left),
            (Face::Four, Direction::Down) => (Face::Six, Direction::Left),
            (Face::Five, Direction::Left) => (Face::Two, Direction::Right),
            (Face::Five, Direction::Up) => (Face::Three, Direction::Right),
            (Face::Five, Direction::Right) => (Face::Four, Direction::Right),
            (Face::Five, Direction::Down) => (Face::Six, Direction::Down),
            (Face::Six, Direction::Left) => (Face::Two, Direction::Down),
            (Face::Six, Direction::Up) => (Face::Five, Direction::Up),
            (Face::Six, Direction::Right) => (Face::Four, Direction::Up),
            (Face::Six, Direction::Down) => (Face::One, Direction::Down),
        }
    }

    fn forward_one(&self) -> Self {
        // And now we implement a version of MizardX@Twitch's approach!

        match (self.direction, self.row % 50, self.col % 50) {
            (Direction::Left, _, 1..) => return *self - (0, 1),
            (Direction::Up, 1.., _) => return *self - (1, 0),
            (Direction::Right, _, ..=48) => return *self + (0, 1),
            (Direction::Down, ..=48, _) => return *self + (1, 0),
            _ => (),
        };

        let (new_face, new_direction) = self.wrap();

        // The next two `match` expressions both come from MizardX@Twitch.
        // I'm not 100% sure I can explain them as well as I'd like. The combination
        // of subtractions in the two `match` statements always cancel each other
        // out unless one direction is `Left` or `Right`, and the other direction
        // is the opposite (i.e., `Right` or `Left`).
        //
        // MizardX was kind enough to continue to try to explain it to me, and I
        // think I understand it better now, if not perfectly.
        //
        // For this first `match` is imagine that we're leaving a face going in the
        // direction being matched against. If we're leaving via Left or Right, the
        // coordinate that matters is the row, and if we're leaving via Up or Down,
        // the coordinate that matters is the column. Regardless of which direction
        // we're going, we can think about the coordinate that matters as going from
        // 0 on our left to 49 on our right. This match converts the value along that
        // number line in front of us to the actual row or column value in the map space.
        // If we're leaving by going left or down, we have to subtract because those
        // numbers are the reverse of the order of the rows or the columns.
        let cw_position = match (self.direction, self.row % 50, self.col % 50) {
            (Direction::Left, r, _) => 49 - r,
            (Direction::Up, _, c) => c,
            (Direction::Right, r, _) => r,
            (Direction::Down, _, c) => 49 - c,
        };

        // This `match` basically inverts the logic in the previous, converting coordinates
        // in map space to number coordinates upon _entering_ a face. Again, `Left` and
        // `Down` need subtractions because their numbers are inverted.
        let (new_row, new_col) = match new_direction {
            Direction::Left => (49 - cw_position, 49),
            Direction::Up => (49, cw_position),
            Direction::Right => (cw_position, 0),
            Direction::Down => (0, 49 - cw_position),
        };

        Self::new(new_row, new_col, new_face, new_direction)
    }

    const fn to_position(self) -> Position {
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
}

impl Map {
    fn empty(num_columns: usize) -> Self {
        Self {
            tiles: Array::from_elem((0, num_columns), Tile::Space),
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
        Ok(())
    }

    fn get_by_position(&self, position: Position) -> Tile {
        // `position` should always be a legal position on the map, so the call
        // to `get()` should always succeed.
        #[allow(clippy::unwrap_used)]
        // println!("Position is {position:?}.");
        *self.tiles.get((position.row, position.col)).unwrap()
    }

    fn forward_one(position: &Position) -> Position {
        position.to_face_position().forward_one().to_position()
    }

    fn forward(&self, mut position: Position, num_steps: u32) -> Position {
        for _ in 0..num_steps {
            let new_position = Self::forward_one(&position);
            let tile = self.get_by_position(new_position);
            // println!("New position is {new_position:?} and tile is {tile:?}.");
            position = match tile {
                Tile::Space => unreachable!("We should never get a space tile. position = {position:?}, direction = {:?}, new_position = {new_position:?}, tile is '{tile}'.", position.direction),
                Tile::Open => new_position,
                Tile::Wall => return position,
            }
        }
        position
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

// If errors occur here, it's because the input file doesn't have the
// specified format, so we'll just call `unwrap()` and panic if things
// don't work out.
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
}

impl You {
    fn new(map: &Map) -> Self {
        let top_row = map.tiles.row(0);
        // The call to `Position` would only return `None` if there were no `Open` tiles
        // on the top row. That shouldn't happen on legal maps, so we'll just `unwrap()`
        // and panic if things aren't legal.
        #[allow(clippy::unwrap_used)]
        let col = top_row.iter().position(|tile| tile == &Tile::Open).unwrap();
        Self {
            position: Position::new(0, col, Direction::Right),
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
            position: Position {
                direction: self.position.direction.turn_left(),
                ..self.position
            },
        }
    }

    const fn turn_right(self) -> Self {
        Self {
            position: Position {
                direction: self.position.direction.turn_right(),
                ..self.position
            },
        }
    }

    fn forward(self, num_steps: u32, map: &Map) -> Self {
        let position = map.forward(self.position, num_steps);
        Self { position }
    }

    const fn row(&self) -> usize {
        self.position.row + 1
    }

    const fn col(&self) -> usize {
        self.position.col + 1
    }

    const fn facing(&self) -> usize {
        self.position.direction as usize
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

    // println!("{map}");

    // println!("{actions:?}");

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
            FacePosition::new(0, 20, Face::Four, Direction::Down)
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
