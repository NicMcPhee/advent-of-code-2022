#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::items_after_test_module)]

use anyhow::{bail, Context};
use std::{fmt::Display, fs, iter::Sum, str::FromStr};

struct Snafu(u64);

impl Snafu {
    const fn into_inner(self) -> u64 {
        self.0
    }

    fn parse_char(c: char) -> anyhow::Result<i64> {
        Ok(match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            ch => bail!("Illegal character {ch}"),
        })
    }

    fn to_char(value: i64) -> anyhow::Result<char> {
        Ok(match value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            value => bail!("Illegal value {value}"),
        })
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .try_fold(0, |current, c| -> anyhow::Result<i64> {
                let char_val = Self::parse_char(c)?;
                Ok(current * 5 + char_val)
            })
            // .and_then(|value| value.try_into().map_err(Into::into))
            .and_then(|value| {
                value
                    .try_into()
                    .context("Failed to convert {value} to a u64")
            })
            .map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_snafu() {
        let input = "1=-0-2";
        #[allow(clippy::unwrap_used)]
        let snafu = Snafu::from_str(input).unwrap();
        assert_eq!(snafu.into_inner(), 1747);
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // #[allow(clippy::cast_possible_wrap)]
        let mut current_value: i64 = self.0 as i64;
        // let mut current_value: i64 = self
        //     .0
        //     .try_into()
        //     .map_err(|error| std::fmt::Error::from(error))?;
        let mut digits = Vec::new();
        while current_value != 0 {
            let digit = (current_value + 2) % 5 - 2;
            // #[allow(clippy::unwrap_used)]
            digits.push(Self::to_char(digit).unwrap());
            current_value = (current_value - digit) / 5;
        }
        for d in digits.into_iter().rev() {
            write!(f, "{d}")?;
        }
        Ok(())
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(Self::into_inner).sum())
    }
}

static INPUT_FILE: &str = "../inputs/day_25.input";

fn main() -> anyhow::Result<()> {
    let total = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(Snafu::from_str)
        .sum::<anyhow::Result<Snafu>>()?;

    println!("The total was {total}");

    Ok(())
}
