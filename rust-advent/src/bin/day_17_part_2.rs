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
    y: u64,
}

impl Position {
    const fn new(x: u8, y: u64) -> Self {
        Self { x, y }
    }

    fn offset(&self, dx: i32, dy: i64) -> Option<Self> {
        let x: u8 = u8::try_from(i32::from(self.x) + dx).ok()?;
        if x >= 7 {
            return None;
        }
        let y: u64 = u64::try_from(i64::try_from(self.y).ok()? + dy).ok()?;
        Some(Self { x, y })
    }

    fn offset_by_position(&self, offset: &Self) -> Option<Self> {
        let dx = i32::from(offset.x);
        let dy = i64::try_from(offset.y).ok()?;
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
            JetDirection::Left => self.move_left(occupied),
            JetDirection::Right => self.move_right(occupied),
        }
    }

    fn move_left(&mut self, occupied: &HashSet<Position>) {
        if self.not_intersects(-1, 0, occupied) {
            // println!("Moving left");
            self.position.x -= 1;
        }
    }

    fn move_right(&mut self, occupied: &HashSet<Position>) {
        if self.not_intersects(1, 0, occupied) {
            // println!("Moving right");
            self.position.x += 1;
        }
    }

    fn not_intersects(&self, dx: i32, dy: i64, occupied: &HashSet<Position>) -> bool {
        self.rock
            .into_iter()
            .any(|p| {
                #[allow(clippy::expect_used)]
                let Some(rock_position) = p.offset_by_position(&self.position) else {
                    return true;
                };
                let Some(offset_position) = rock_position.offset(dx, dy) else {
                    return true;
                };
                occupied.contains(&offset_position)
            })
            .not()
    }

    fn drop(&mut self, occupied: &HashSet<Position>) -> bool {
        if self.position.y > 1 && self.not_intersects(0, -1, occupied) {
            self.position.y -= 1;
            // println!("Dropping");
            return true;
        }
        false
    }
}

struct Chamber {
    jet_directions: Cycle<IntoIter<JetDirection>>,
    occupied: HashSet<Position>,
    highest_rock_point: u64,
    rock_iter: Cycle<RockIter>,
}

// TODO: Implement this to help with debugging.
impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.highest_rock_point).rev() {
            for x in 0..7 {
                let position = Position::new(x, y);
                if self.occupied.contains(&position) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "The highest rock is at {}", self.highest_rock_point)?;
        Ok(())
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

    fn drop_rocks(&mut self, num_rocks: u64) {
        for n in 0..num_rocks {
            let mut positioned_rock = self.next_rock();
            self.drop_rock(&mut positioned_rock);
            // println!("{self}\n\n");
            if n % 1_000_000 == 0 {
                println!("We're at rock {n}");
            }
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
                y: self.highest_rock_point + 4,
            },
        }
    }

    fn print_top_and_bottom_lines(&self) {
        println!("Top line");
        for x in 0..7 {
            let position = Position::new(x, self.highest_rock_point);
            if self.occupied.contains(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
        for x in 0..7 {
            let position = Position::new(x, 1);
            if self.occupied.contains(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("\nBottom line");
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

    let num_jet_directions = jet_directions.len();
    let num_jet_directions: u64 = u64::try_from(num_jet_directions)?;
    // let cycle_size = num_jet_directions * 5;

    println!("There are {num_jet_directions} jet directions");

    let mut chamber = Chamber::new(jet_directions);

    // chamber.drop_rocks(15);
    // println!("{chamber}");
    // chamber.drop_rocks(35);
    // println!("{chamber}");
    // chamber.drop_rocks(35);
    // println!("{chamber}");

    // chamber.drop_rocks(cycle_size);

    // chamber.print_top_and_bottom_lines();

    // chamber.drop_rocks(cycle_size);

    // chamber.print_top_and_bottom_lines();

    // let block_height = chamber.highest_rock_point;
    // println!("The block height is {block_height}");

    // let num_rocks: u64 = 1_000_000_000_000;

    chamber.drop_rocks(1_000_000);
    // chamber.drop_rocks(num_rocks);
    // let result = block_height * (num_blocks / cycle_size);
    let result = chamber.highest_rock_point;

    println!("Result is {result}");

    Ok(())
}
