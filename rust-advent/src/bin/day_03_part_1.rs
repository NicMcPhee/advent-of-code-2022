#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashSet, fs};

use anyhow::{bail, Context, Result};

static INPUT_FILE: &str = "../inputs/day_03.input";

fn main() -> Result<()> {
    let sum_of_priorities = process_rucksacks(INPUT_FILE)?;

    println!("The sum of the priorities is {sum_of_priorities:?}");

    Ok(())
}

fn process_rucksacks(input_file: &str) -> Result<u32> {
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open file '{input_file}'"))?;

    contents.lines().map(process_rucksack).sum()
}

fn process_rucksack(line: &str) -> Result<u32> {
    let (first, second) = line.split_at(line.len() / 2);
    assert_eq!(first.len(), second.len());
    let first_set = first.chars().collect::<HashSet<_>>();
    let second_set = second.chars().collect::<HashSet<_>>();

    // The problem statement ensures that there is exactly one shared item.
    let shared_item = first_set
        .intersection(&second_set)
        .next()
        .with_context(|| format!("There was no common character in '{first}' and '{second}'"))?;
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
