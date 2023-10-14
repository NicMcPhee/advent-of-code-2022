#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::items_after_test_module)]

use anyhow::Context;
use std::fs;

static INPUT_FILE: &str = "../inputs/day_25_test.input";

fn main() -> anyhow::Result<()> {
    let lines: Vec<&str> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .collect();

    Ok(())
}
