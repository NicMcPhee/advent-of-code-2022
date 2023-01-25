#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_03.input";

fn main() -> Result<()> {
    let sum_of_priorities = process_rucksacks(INPUT_FILE)?;

    println!("The sum of the priorities is {sum_of_priorities:?}");

    Ok(())
}

fn process_rucksacks(input_file: &str) -> Result<usize> {
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open file '{input_file}'"))?;

    todo!()
}