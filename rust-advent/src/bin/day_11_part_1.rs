#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{fs, str::FromStr};

use anyhow::{bail, Context, Result};

static INPUT_FILE: &str = "../inputs/day_11.input";

struct Monkey {}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

fn main() -> Result<()> {
    let monkeys = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?;

    let monkey_business: usize = todo!();

    println!("The monkey business was {monkey_business}");

    Ok(())
}
