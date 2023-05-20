#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashSet, fs, iter::Cycle, ops::Not, vec::IntoIter, fmt::Display};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use anyhow::{Context, bail};

#[derive(Clone)]
enum JetDirection {
    Left,
    Right,
}

impl TryFrom<char> for JetDirection {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        Ok(match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => bail!("Tried to convert illegal character '{c}' into JetDirection"),
        })
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: u8,
    y: u32,
}

impl Position {
    const fn new(x: u8, y: u32) -> Self {
        Self { x, y }
    }

    fn offset(&self, dx: i32, dy: i32) -> Option<Self> {
        let x: u8 = u8::try_from(i32::from(self.x) + dx).ok()?;
        if x >= 7 {
            return None;
        }
        let y: u32 = u32::try_from(i32::try_from(self.y).ok()? + dy).ok()?;
        Some(Self { x, y })
    }

    fn offset_by_position(&self, offset: &Position) -> Option<Self> {
        let dx = i32::from(offset.x);
        let dy = i32::try_from(offset.y).ok()?;
        self.offset(dx, dy)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

impl Rock {
    fn horizontal_iter() -> IntoIter<Position> {
        (0..4)
            .map(|x| Position::new(x, 0))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn plus_iter() -> IntoIter<Position> {
        vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 1),
        ]
        .into_iter()
    }

    fn l_iter() -> IntoIter<Position> {
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ]
        .into_iter()
    }

    fn vertical_iter() -> IntoIter<Position> {
        (0..4)
            .map(|y| Position::new(0, y))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn square_iter() -> IntoIter<Position> {
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
        ]
        .into_iter()
    }
}

impl IntoIterator for Rock {
    type Item = Position;

    type IntoIter = std::vec::IntoIter<Position>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Horizontal => Self::horizontal_iter(),
            Self::Plus => Self::plus_iter(),
            Self::L => Self::l_iter(),
            Self::Vertical => Self::vertical_iter(),
            Self::Square => Self::square_iter(),
        }
    }
}

struct PositionedRock {
    rock: Rock,
    position: Position,
}

impl PositionedRock {
    fn shift(&mut self, direction: &JetDirection, occupied: &HashSet<Position>) {
        match direction {
            JetDirection::Left if self.position.x > 0 => self.move_left(occupied),
            JetDirection::Right if self.position.x < 6 => self.move_right(occupied),
            _ => (),
        }
    }

    fn move_left(&mut self, occupied: &HashSet<Position>) {
        if self.not_intersects(-1, 0, occupied) {
            self.position.x -= 1;
        }
    }

    fn move_right(&mut self, occupied: &HashSet<Position>) {
        if self.not_intersects(1, 0, occupied) {
            self.position.x += 1;
        }
    }

    fn not_intersects(&self, dx: i32, dy: i32, occupied: &HashSet<Position>) -> bool {
        self.rock
            .into_iter()
            .any(|p| {
                // These type conversion errors shouldn't happen if we have the logic
                // correct.
                #[allow(clippy::expect_used)]
                occupied.contains(&Position {
                    x: u8::try_from(i32::from(p.x + self.position.x) + dx)
                        .expect("x coordinate didn't fit in u8"),
                    y: u32::try_from(
                        i32::try_from(p.y + self.position.y)
                            .expect("y coordinate couldn't convert to i32")
                            + dy,
                    )
                    .expect("y coordinate didn't fit in u32"),
                })
            })
            .not()
    }

    fn drop(&mut self, occupied: &HashSet<Position>) -> bool {
        if self.position.y > 0 && self.not_intersects(0, -1, occupied) {
            self.position.y -= 1;
            return true;
        }
        false
    }
}

struct Chamber {
    jet_directions: Cycle<IntoIter<JetDirection>>,
    occupied: HashSet<Position>,
    highest_rock_point: u32,
    rock_iter: Cycle<RockIter>,
}

// TODO: Implement this to help with debugging.
impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Chamber {
    fn new(jet_directions: Vec<JetDirection>) -> Self {
        Self {
            jet_directions: jet_directions.into_iter().cycle(),
            occupied: HashSet::new(),
            highest_rock_point: 0,
            rock_iter: Rock::iter().cycle(),
        }
    }

    fn drop_rocks(&mut self, num_rocks: u32) {
        for _ in 0..num_rocks {
            let mut positioned_rock = self.next_rock();
            self.drop_rock(&mut positioned_rock);
        }
    }

    fn drop_rock(&mut self, rock: &mut PositionedRock) {
        loop {
            rock.shift(
                #[allow(clippy::expect_used)]
                &self.jet_directions
                    .next()
                    .expect("We should never reach the end of jet directions because of `cycle`"),
                &self.occupied,
            );
            if rock.drop(&self.occupied).not() {
                break;
            }
        }
        let new_positions = rock.rock.into_iter().map(|p| Position {
            x: p.x + rock.position.x,
            y: p.y + rock.position.y,
        });
        for p in new_positions {
            if p.y > self.highest_rock_point {
                self.highest_rock_point = p.y;
            }
            self.occupied.insert(p);
        }
    }

    fn next_rock(&mut self) -> PositionedRock {
        #[allow(clippy::expect_used)]
        let rock = self
            .rock_iter
            .next()
            .expect("We should never reach the end of rocks because of `cycle`");
        PositionedRock {
            rock,
            position: Position {
                x: 2,
                y: self.highest_rock_point + 3,
            },
        }
    }
}

static INPUT_FILE: &str = "../inputs/day_17_test.input";

fn main() -> anyhow::Result<()> {
    let jet_directions = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .chars()
        .map(TryInto::<JetDirection>::try_into)
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut chamber = Chamber::new(jet_directions);

    chamber.drop_rocks(2022);

    println!("The tower height is {}", chamber.highest_rock_point);

    Ok(())
}
