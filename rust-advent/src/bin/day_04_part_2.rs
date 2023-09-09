#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{Context, Result};
use std::{fs, str::FromStr};

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
        // Thanks to @NathanielBumppo@Twitch for pointing out
        // that we can use `usize::from()` to convert from a boolean
        // to a `usize` in exactly the desired way.
        Ok(current_count + usize::from(partially_overlaps(line)?))
    })
}

struct SectionAssignment {
    start: usize,
    end: usize,
}

impl FromStr for SectionAssignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_once('-')
            .with_context(|| format!("There was no dash in the range '{s}'"))?;

        // @ikopor@Twitch suggested using the `enum_map` crate. We would make
        // an `enum` with `Start` and `End` and then use `EnumMap` to convert
        // to a `SectionAssignment`. This would let us stick with iterators
        // instead of dropping out here to handle `start` and `end`
        // separately. Maybe a big heavyweight for here, but I might want
        // to explore it since the `enum_map` crate sounds useful.

        let start: usize = start
            .parse()
            .with_context(|| format!("Couldn't parse '{start}' to an int"))?;

        let end: usize = end
            .parse()
            .with_context(|| format!("Couldn't parse '{end}' to an int"))?;

        Ok(Self { start, end })
    }
}

impl SectionAssignment {
    const fn contains_endpoint_of(&self, other: &Self) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }

    const fn contains(&self, section: usize) -> bool {
        self.start <= section && section <= self.end
    }
}

fn partially_overlaps(line: &str) -> Result<bool> {
    let (first, second) = line
        .split_once(',')
        .with_context(|| format!("There was no comma in the line '{line}'"))?;

    let first: SectionAssignment = first.parse()?;
    let second: SectionAssignment = second.parse()?;

    Ok(first.contains_endpoint_of(&second) || second.contains_endpoint_of(&first))
}
