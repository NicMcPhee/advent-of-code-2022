#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u32,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    adjacent_valve_names: Vec<String>,
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn valve(line: &str) -> IResult<&str, Valve> {
    let (s, name) = preceded(tag("Valve "), take(2usize))(line)?;
    let (s, flow_rate) = preceded(tag(" has flow rate="), u32)(s)?;
    let (s, adjacent_valve_names) = all_consuming(preceded(
        alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        )),
        separated_list1(tag(", "), map(take(2usize), std::string::ToString::to_string)),
    ))(s)?;

    Ok((
        s,
        Valve {
            name: name.to_string(),
            flow_rate,
            adjacent_valve_names,
        },
    ))
}

fn extract_valve(line: &str) -> anyhow::Result<Valve> {
    // This version might actually be easier to understand, although Clippy
    // reasonably prefers not adding a closure unnecessarily.
    //     Ok(valve(line).map_err(|e| e.to_owned())?.1)
    Ok(valve(line).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?.1)
struct BitSet {
    bits: u64,
}

impl BitSet {
    #[must_use]
    fn add(&self, position: u16) -> Self {
        assert!(position < 64);
        let bit = 1u64 << position;
        Self {
            bits: self.bits | bit,
        }
    }

    #[must_use]
    fn contains(&self, position: u16) -> bool {
        assert!(position < 64);
        let bit = 1u64 << position;
        self.bits & bit > 0
    }
}

#[cfg(test)]
mod bit_set_tests {
    use super::*;

    #[test]
    fn empty_bit_set() {
        let empty = BitSet { bits: 0 };
        for i in 0..64 {
            assert!(empty.contains(i).not());
        }
    }

    #[test]
    fn full_bit_set() {
        let mut bits = BitSet { bits: 0 };
        for i in 0..64 {
            bits = bits.add(i);
        }
        for i in 0..64 {
            assert!(bits.contains(i));
        }
    }

    #[test]
    fn only_even_bits() {
        let mut bits = BitSet { bits: 0 };
        for i in 0..32 {
            bits = bits.add(i * 2);
        }
        for i in 0..64 {
            assert_eq!(bits.contains(i), i % 2 == 0);
        }
    }
}
}

static INPUT_FILE: &str = "../inputs/day_16_test.input";

fn main() -> anyhow::Result<()> {
    let valves = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(extract_valve)
        .collect::<anyhow::Result<Vec<Valve>>>()?;

    println!("{valves:?}");

    Ok(())
}
