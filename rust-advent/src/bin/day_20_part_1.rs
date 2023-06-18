#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;

static INPUT_FILE: &str = "../inputs/day_20_test.input";

fn main() -> anyhow::Result<()> {
    let values: Vec<i16> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i16>, _>>()?;

    println!("The values are {values:?}");

    Ok(())
}
