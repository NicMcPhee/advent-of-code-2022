#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    str::FromStr,
};

use anyhow::{bail, Context, Result};

static INPUT_FILE: &str = "../inputs/day_08.input";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    todo!();

    // println!("The total of the sizes was {total_sizes}");

    // Ok(())
}
