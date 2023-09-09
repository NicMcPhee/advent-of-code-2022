#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context, Result};
use std::{fs, str::FromStr};

static INPUT_FILE: &str = "../inputs/day_02.input";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Rps {
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

impl TryFrom<i8> for Rps {
    type Error = anyhow::Error;

    fn try_from(i: i8) -> Result<Self> {
        match i {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            _ => bail!("Can't convert {i} to an RPS value"),
        }
    }
}

// TODO: Benchmark passing `self` (i.e., an 8-bit value) instead
//   of `&self` (i.e., a 64-bit value). Does it really make a
//   difference? Clippy seemed to think so, but it would be interesting
//   to see what the actual impact is.
impl Rps {
    fn game_score(self, their_move: Self) -> u32 {
        if self.beats() == their_move {
            6
        } else if self == their_move {
            3
        } else {
            0
        }
    }

    fn shift(self, shift_amount: i8) -> Self {
        let our_val = self as i8;
        let mut their_val = (3 + our_val + shift_amount) % 3;
        if their_val == 0 {
            their_val = 3;
        }
        assert!((1..=3).contains(&their_val));
        #[allow(clippy::unwrap_used)]
        their_val.try_into().unwrap()
    }

    fn beats(self) -> Self {
        self.shift(-1)
    }

    fn loses_to(self) -> Self {
        self.shift(1)
    }

    // This takes their move (self) and the desired outcome
    // and returns the move we would need to make to generate
    // that result.
    fn our_move(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => self,
            Outcome::Win => self.loses_to(),
            Outcome::Lose => self.beats(),
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

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let total_score = contents.lines().map(process_game).sum::<Result<u32>>()?;

    println!("The total score was {total_score}");

    Ok(())
}

fn process_game(line: &str) -> Result<u32> {
    let mut parts = line.split_ascii_whitespace();

    let their_move = parts
        .next()
        .with_context(|| format!("Missing first move on line '{line}'"))?
        .parse::<Rps>()?;

    let outcome = parts
        .next()
        .with_context(|| format!("Missing outcome on line '{line}'"))?
        .parse::<Outcome>()?;

    let our_move = their_move.our_move(outcome);

    Ok(our_move.game_score(their_move) + (our_move as u32))
}

#[cfg(test)]
mod beats_tests {
    use super::*;

    #[test]
    fn beats_check() {
        assert_eq!(Rps::Rock.beats(), Rps::Scissors);
        assert_eq!(Rps::Paper.beats(), Rps::Rock);
        assert_eq!(Rps::Scissors.beats(), Rps::Paper);
    }
}

#[cfg(test)]
mod score_tests {
    use super::*;

    #[test]
    fn we_play_rock() {
        assert_eq!(3, Rps::Rock.game_score(Rps::Rock));
        assert_eq!(0, Rps::Rock.game_score(Rps::Paper));
        assert_eq!(6, Rps::Rock.game_score(Rps::Scissors));
    }
}
