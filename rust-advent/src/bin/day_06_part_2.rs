#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashSet, fs};

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_06.input";

const WINDOW_SIZE: usize = 14;

// TODO: Try using Rayon's `par_windows()` and do some benchmarking
//  to see how much faster that makes things.
fn find_marker(s: &str) -> Result<usize> {
    let (pos, _) = s
        .as_bytes()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_, cs)| all_unique(cs))
        .context("There were no unique windows in the input")?;

    // The marker is four characters ahead of the start of the first block
    // of unique characters.
    Ok(pos + WINDOW_SIZE)
}

fn all_unique(cs: &[u8]) -> bool {
    // This works because hashing on &T is converting to hashing
    // on T, so we end up hashing on the underlying `u8`s (and
    // not their addresses as I had thought we would), which
    // makes everything work.
    // See https://doc.rust-lang.org/std/hash/trait.Hash.html#impl-Hash-for-%26T
    // and click on the `source` to see what actually happens.
    // Thanks to @esitsu for pointing this out, in response to a
    // question from @ikopor.
    let unique_chars: HashSet<&u8> = cs.iter().collect();
    cs.len() == unique_chars.len()
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let marker = find_marker(&contents)?;

    println!("The marker was {marker}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn third_example() -> Result<()> {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = find_marker(input)?;
        assert_eq!(23, result);
        Ok(())
    }
}
