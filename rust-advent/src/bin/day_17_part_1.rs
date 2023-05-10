#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashMap, fmt::Display, fs, ops::Not};

use anyhow::Context;

static INPUT_FILE: &str = "../inputs/day_17_test.input";

fn main() -> anyhow::Result<()> {
    let jet_directions = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let result: u32 = todo!();

    println!("The tower height is {result}");

    Ok(())
}
