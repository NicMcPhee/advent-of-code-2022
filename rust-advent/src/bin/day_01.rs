use anyhow::{Context, Result};
use std::fs;
use std::str::SplitAsciiWhitespace;

fn main() -> Result<()> {
    let input_file = "../inputs/day_01.input";
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open the input file '{input_file}'"))?;

    let mut totals: Vec<usize> = contents
        .split("\n\n")
        .map(|g| g.split_ascii_whitespace())
        .map(sum_group)
        .collect::<Result<Vec<_>, _>>()?;

    let biggest = totals.iter().max().unwrap();

    println!("The largest sum was {biggest}");

    totals.sort();
    let biggest_three: usize = totals.iter().rev().take(3).sum();

    println!("The sum of the three largest values was {biggest_three}.");

    Ok(())
}

fn sum_group(group: SplitAsciiWhitespace) -> Result<usize> {
    group
        .map(|s| {
            s.parse::<usize>()
                .with_context(|| format!("Failed to parse '{}' to `usize`.", s))
        })
        .sum::<Result<usize>>()
}
