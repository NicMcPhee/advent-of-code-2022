#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context, Result};
use itertools::{Chunk, Itertools};
use std::{collections::HashSet, fs, str::Lines};

static INPUT_FILE: &str = "../inputs/day_03.input";

fn main() -> Result<()> {
    let sum_of_priorities = process_groups(INPUT_FILE)?;

    println!("The sum of the priorities is {sum_of_priorities:?}");

    Ok(())
}

fn process_groups(input_file: &str) -> Result<u32> {
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open file '{input_file}'"))?;

    contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(process_group)
        .sum()
}

fn process_group(group: Chunk<Lines>) -> Result<u32> {
    let shared_items = group
        .map(|s: &str| s.chars().collect::<HashSet<_>>())
        .reduce(|so_far, other| so_far.intersection(&other).copied().collect())
        .with_context(|| "There were no lines in a group")?;

    let shared_item = shared_items
        .iter()
        .next()
        .with_context(|| "There was no shared item in a group")?;
    let shared_item = *shared_item;

    Ok(match shared_item {
        'a'..='z' => char_to_priority(shared_item, 'a', 1),
        'A'..='Z' => char_to_priority(shared_item, 'A', 27),
        _ => bail!("Illegal shared item '{}'", shared_item),
    })
}

const fn char_to_priority(c: char, offset_char: char, offset_val: u32) -> u32 {
    c as u32 - (offset_char as u32) + offset_val
}
