#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_04.input";

fn main() -> Result<()> {
    let num_overlapping_assignments = process_pairs(INPUT_FILE)?;

    println!("The number of overlapping assignments is {num_overlapping_assignments:?}");

    Ok(())
}

fn process_pairs(input_file: &str) -> Result<usize> {
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open file '{input_file}'"))?;

    contents.lines().try_fold(0, |current_count, line| {
        // Alternatively (thanks to @NathanielBumppo@Twitch),
        //  current_count + overlaps(line)?.into()
        // This converts a boolean to 0 or 1 in the way we want.
        Ok(current_count + usize::from(overlaps(line)?))
        // Ok(if overlaps(line)? {
        //     current_count + 1
        // } else {
        //     current_count
        // })
    })
}

fn overlaps(line: &str) -> Result<bool> {
    let (first, second) = line
        .split_once(',')
        .with_context(|| format!("There was no comma in the line '{line}'"))?;

    todo!()
}
