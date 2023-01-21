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
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
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

    // This takes their move (self) and the desired outcome
    // and returns the move we would need to make to generate
    // that result.
    const fn our_move(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => self,
            Outcome::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
            Outcome::Lose => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            }
        }
    }
}

// If we don't derive Copy/Clone, then Clippy recommends passing
// a reference to `our_move` up above. I'd like to understand
// that a bit better.
#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(c: &str) -> Result<Self> {
        match c {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => bail!("Illegal character '{c}' for Outcome"),
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
        .split_ascii_whitespace();

    let their_move = parts
        .next()
        .with_context(|| format!("Missing first move on line '{line}'"))?
        .parse::<RPS>()?;

    let outcome = parts
        .next()
        .with_context(|| format!("Missing outcome on line '{line}'"))?
        .parse::<Outcome>()?;

    let our_move = their_move.our_move(outcome);

    Ok(our_move.game_score(their_move) + (our_move as u32))
}
