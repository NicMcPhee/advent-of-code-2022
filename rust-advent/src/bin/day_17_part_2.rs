#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashMap, fs, iter::{Cycle, Enumerate}, ops::Not, vec::IntoIter, fmt::Display};

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
    fn shift(&mut self, direction: &JetDirection, occupied: &HashMap<Position, RockJetIndex>) {
        match direction {
            JetDirection::Left => self.move_left(occupied),
            JetDirection::Right => self.move_right(occupied),
        }
    }

    fn move_left(&mut self, occupied: &HashMap<Position, RockJetIndex>) {
        if self.not_intersects(-1, 0, occupied) {
            // println!("Moving left");
            self.position.x -= 1;
        }
    }

    fn move_right(&mut self, occupied: &HashMap<Position, RockJetIndex>) {
        if self.not_intersects(1, 0, occupied) {
            // println!("Moving right");
            self.position.x += 1;
        }
    }

    fn not_intersects(&self, dx: i32, dy: i64, occupied: &HashMap<Position, RockJetIndex>) -> bool {
        self.rock
            .into_iter()
            .any(|p| {
                let Some(rock_position) = p.offset_by_position(&self.position) else {
                    return true;
                };
                let Some(offset_position) = rock_position.offset(dx, dy) else {
                    return true;
                };
                occupied.contains_key(&offset_position)
            })
            .not()
    }

    fn drop(&mut self, occupied: &HashMap<Position, RockJetIndex>) -> bool {
        if self.position.y > 1 && self.not_intersects(0, -1, occupied) {
            self.position.y -= 1;
            // println!("Dropping");
            return true;
        }
        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RockJetIndex {
    rock: usize,
    jet: usize,
}

struct Chamber {
    jet_directions: Cycle<Enumerate<IntoIter<JetDirection>>>,
    occupied: HashMap<Position, RockJetIndex>,
    highest_rock_point: u64,
    rock_iter: Cycle<RockIter>,
}

// TODO: Implement this to help with debugging.
impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.highest_rock_point).rev() {
            for x in 0..7 {
                let position = Position::new(x, y);
                if let Some(rji) = self.occupied.get(&position) {
                    write!(f, "{}", rji.rock)?;
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
            jet_directions: jet_directions.into_iter().enumerate().cycle(),
            occupied: HashMap::new(),
            highest_rock_point: 0,
            rock_iter: Rock::iter().cycle(),
        }
    }

    fn drop_rocks(&mut self, num_rocks: u64) {
        let mut found_start = false;
        let mut found_end = false;
        for n in 0..num_rocks {
            let mut positioned_rock = self.next_rock();
            self.drop_rock(&mut positioned_rock);
            if !found_start && self.highest_rock_point >= 2693 {
                println!("Rock number {n} got us to row 2693");
                found_start = true;
            }
            if !found_end && self.highest_rock_point >= 2693+2694 {
                println!("Rock number {n} got us to row 2694");
                found_end = true;
            }
            // println!("{self}\n\n");
            if n % 1_000_000 == 0 {
                println!("We're at rock {n}");
            }
        }
    }

    fn drop_rock(&mut self, rock: &mut PositionedRock) {
        let mut jet_index: usize;
        let mut jet: JetDirection;
        #[allow(clippy::expect_used)]
        loop {
            (jet_index, jet) = self.jet_directions
                                .next()
                                .expect("We should never reach the end of jet directions because of `cycle`");
            rock.shift(
                &jet,
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
            let rji = RockJetIndex {
                rock: rock.rock as usize,
                jet: jet_index,
            };
            self.occupied.insert(p, rji);
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

    #[allow(dead_code)]
    fn print_top_and_bottom_lines(&self) {
        println!("Top line");
        for x in 0..7 {
            let position = Position::new(x, self.highest_rock_point);
            if self.occupied.contains_key(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
        for x in 0..7 {
            let position = Position::new(x, 1);
            if self.occupied.contains_key(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("\nBottom line");
    }

    fn row(&self, row_num: u64) -> [Option<RockJetIndex>; 7] {
        let mut result = [None; 7];
        for x in 0..7 {
            let position = Position { x, y: row_num };
            result[x as usize] = self.occupied.get(&position).copied();
        }
        result
    }

    fn find_cycle(&self) -> Option<(u64, u64)> {
        // Assume we've dropped a bunch of rocks and run the tortoise/hare algorithm.
        let mut slow = 0;
        let mut fast = 1;
        while self.row(slow) != self.row(fast) {
            slow += 1;
            fast += 2;
            if fast > self.highest_rock_point {
                return None
            }
        }
        println!("Slow = {slow}, fast = {fast}");
        println!("{:?}", self.row(slow));
        println!("{:?}", self.row(fast));
        Some((slow, fast-slow))
    }
}

static INPUT_FILE: &str = "../inputs/day_17.input";

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

    // chamber.drop_rocks(10_000);
    chamber.drop_rocks(1742 + 1583);
    println!("Height after {} rocks is {}", 1742 + 1583, chamber.highest_rock_point);

    // 35 rocks is the cycle length on the test data.
    // Those 35 rocks add 53 rows to the height.
    // We start the cycle at row 52.
    // 31 + K*35 + 19 = 1,000,000,000,000
    // The largest K is 28,571,428,570
    // The height will be 78 + 53*28,571,428,570 = 1,514,285,714,288

    // Cycle starts on rock 1,742 and ends on rock 3,467.
    // So 1,725 rocks in the cycle on the "real" data.
    // Those 1,725 rocks add 2694 rows to the height.
    // We start the cycle at row 2693.
    // 1,742 + K*1,725 + ??? = 1,000,000,000,000
    // The largest K is 579,710,143 with 1,583 left over.
    // 1742 1583
    // The height will be 5149 + 2694 * 579,710,143 = 1,561,739,130,391

    if let Some((start, length)) = chamber.find_cycle() {
        println!("We found a cycle that starts at {start} and has length {length}.");
    } else {
        println!("We failed to find a cycle!");
    }
    let result = chamber.highest_rock_point;

    println!("Result is {result}");

    Ok(())
}
