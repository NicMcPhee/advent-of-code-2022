#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context};
use ndarray::{Array, Array2, ArrayView};
use std::{fs, str::FromStr};

#[derive(Clone, Debug)]
enum Tile {
    Space,
    Open,
    Wall,
}

#[derive(Debug)]
struct Map {
    tiles: Array2<Tile>,
}

// TODO: `impl Display for Map` so we can see if we're parsing the map
//   correctly.

impl Map {
    fn empty(num_columns: usize) -> Self {
        Self {
            tiles: Array::from_elem((0, num_columns), Tile::Space),
        }
    }

    // TODO: Pad the row on the right with Tile::Space when it's shorter
    // than the `tiles` array.
    fn add_row(&mut self, row: &str) -> anyhow::Result<()> {
        let tiles = row
            .chars()
            .map(|c| {
                Ok(match c {
                    ' ' => Tile::Space,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => bail!("We found an illegal tile character: '{c}."),
                })
            })
            .collect::<anyhow::Result<Vec<Tile>>>()?;
        println!("About to push {} tiles", tiles.len());
        self.tiles.push_row(ArrayView::from(&tiles))?;
        Ok(())
    }
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

// Maybe re-do this with `nom`? It would handle the edge cases and error handling much more
// cleanly than I am here.
impl FromStr for Directions {
    type Err = anyhow::Error;

    #[allow(clippy::unwrap_used)]
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let moves = s
            .split_inclusive("LR")
            .flat_map(|s| {
                // I think this logic is problematic. We repeat the parsing logic
                // three times, and we create a ton of little vectors, which isn't
                // pretty. We also have three `unwrap()`s, which is icky.
                if let Some(n_str) = s.strip_suffix('L') {
                    let n = str::parse::<i32>(n_str).unwrap();
                    return vec![Move::Forward(n), Move::Left];
                } else if let Some(n_str) = s.strip_suffix('R') {
                    let n = str::parse::<i32>(n_str).unwrap();
                    return vec![Move::Forward(n), Move::Right];
                }
                let n = str::parse::<i32>(s).unwrap();
                vec![Move::Forward(n)]
            })
            .collect::<Vec<Move>>();
        Ok(Self { moves })
    }
}

static INPUT_FILE: &str = "../inputs/day_22_test.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;
    let mut lines = file.lines().peekable();

    let width = lines.peek().context("Couldn't find a first line")?.len();
    let mut map = Map::empty(width);

    // TODO: Figure out why `&mut` was needed here to prevent the loop from taking ownership.
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        // parse line and add to array
        map.add_row(line)?;
    }

    println!("{map:?}");

    let directions: Directions = str::parse(
        lines
            .next()
            .context("Failed to parse the directions line")?,
    )?;

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
