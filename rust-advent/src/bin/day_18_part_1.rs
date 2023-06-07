#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::{bail, Context};

static INPUT_FILE: &str = "../inputs/day_18_test.input";

fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim();

    Ok(())
}
