#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    str::FromStr,
};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Terrain {
    heights: Vec<Vec<char>>,
}

impl FromStr for Terrain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

static INPUT_FILE: &str = "../inputs/day_12_test.input";

fn main() -> Result<()> {
    let terrain = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .parse::<Terrain>()?;

    todo!();

    Ok(())
}
