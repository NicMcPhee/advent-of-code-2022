#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail};

static INPUT_FILE: &str = "../inputs/day_02.input";

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RPS {
    type Err = anyhow::Error;

    fn from_str(c: &str) -> Result<Self> {
        match c {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("Illegal character '{c}' for Rock-Paper-Scissors"),
        }
    }
}

// TODO: Benchmark passing `self` (i.e., an 8-bit value) instead
//   of `&self` (i.e., a 64-bit value). Does it really make a
//   difference? Clippy seemed to think so, but it would be interesting
//   to see what the actual impact is.
impl RPS {
    const fn game_score(self, their_move: Self) -> u32 {
        let our_val = self as i8;
        let their_val = their_move as i8;
        if (3 + our_val - their_val) % 3 == 1 {
            6
        } else if our_val == their_val {
            3
        } else {
            0
        }
    }
}

#[cfg(test)]
mod score_tests {
    use super::*;

    #[test]
    fn we_play_rock() {
        assert_eq!(3, RPS::Rock.game_score(RPS::Rock));
        assert_eq!(0, RPS::Rock.game_score(RPS::Paper));
        assert_eq!(6, RPS::Rock.game_score(RPS::Scissors));
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let total_score = contents
        .lines()
        .map(process_game)
        .sum::<Result<u32>>()?;
    
    println!("The total score was {total_score}");

    Ok(())
}

fn process_game(line: &str) -> Result<u32> {
    let mut parts = line
        .split_ascii_whitespace()
        .map(RPS::from_str);
    let their_move = parts
        .next()
        .with_context(|| format!("Missing first move on line '{line}'"))??;
    let our_move = parts
        .next()
        .with_context(|| format!("Missing second move on line '{line}'"))??;

    Ok(our_move.game_score(their_move) + (our_move as u32))
}
