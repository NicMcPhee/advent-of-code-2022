#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashMap, fmt::Display, fs, ops::Not};

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
        separated_list1(
            tag(", "),
            map(take(2usize), std::string::ToString::to_string),
        ),
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
    Ok(valve(line)
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?
        .1)
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct BitSet {
    bits: u64,
}

impl BitSet {
    #[must_use]
    fn insert(&self, position: u8) -> Self {
        assert!(position < 64);
        let bit = 1u64 << position;
        Self {
            bits: self.bits | bit,
        }
    }

    #[must_use]
    fn contains(&self, position: u8) -> bool {
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
            bits = bits.insert(i);
        }
        for i in 0..64 {
            assert!(bits.contains(i));
        }
    }

    #[test]
    fn only_even_bits() {
        let mut bits = BitSet { bits: 0 };
        for i in 0..32 {
            bits = bits.insert(i * 2);
        }
        for i in 0..64 {
            assert_eq!(bits.contains(i), i % 2 == 0);
        }
    }
}

#[derive(Debug)]
struct NumberedValve {
    number: u8,
    valve: Valve,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    my_valve_name: String,
    elephant_valve_name: String,
    open_valves: BitSet,
    time_remaining: u8,
}

impl State {
    fn new<'a>(
        mut name: &'a str,
        mut other_name: &'a str,
        open_valves: BitSet,
        time_remaining: u8,
    ) -> Self {
        if name > other_name {
            (name, other_name) = (other_name, name);
        }
        Self {
            my_valve_name: name.to_string(),
            elephant_valve_name: other_name.to_string(),
            open_valves,
            time_remaining,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {} : {:b} : {}",
            self.my_valve_name,
            self.elephant_valve_name,
            self.open_valves.bits,
            self.time_remaining
        )
    }
}

struct Cave {
    names_to_valves: HashMap<String, NumberedValve>,
}

impl Cave {
    fn new(valves: Vec<Valve>) -> Self {
        let mut names_to_valves = HashMap::new();
        assert!(
            valves.len() < 64,
            "Too many valves ({}) to fit in a u8",
            valves.len()
        );
        for (number, v) in valves.into_iter().enumerate() {
            let valve_name = v.name.clone();
            let numbered_valve = NumberedValve {
                #[allow(clippy::cast_possible_truncation)]
                number: number as u8,
                valve: v,
            };
            names_to_valves.insert(valve_name, numbered_valve);
        }
        Self { names_to_valves }
    }

    fn numbered_valve(&self, name: &str) -> anyhow::Result<&NumberedValve> {
        self.names_to_valves.get(name).with_context(|| {
            format!(
                "Didn't find {} in the `names_to_valves` map {:?}",
                name, self.names_to_valves
            )
        })
    }

    fn process_name(
        &self,
        name: &str,
        other_name: &str,
        time_remaining: u8,
        open_valves: &mut BitSet,
        known_results: &mut HashMap<State, u32>,
    ) -> anyhow::Result<Vec<u32>> {
        let valve = self.numbered_valve(name)?;
        let mut recursive_values = Vec::with_capacity(1 + valve.valve.adjacent_valve_names.len());
        if valve.valve.flow_rate > 0 && open_valves.contains(valve.number).not() {
            let new_state = State::new(
                name,
                other_name,
                open_valves.insert(valve.number),
                time_remaining,
            );
            recursive_values.push(
                valve.valve.flow_rate * u32::from(time_remaining)
                    + self.max_release(new_state, known_results)?,
            );
        }
        for valve_name in &valve.valve.adjacent_valve_names {
            let new_state = State::new(valve_name, other_name, open_valves.clone(), time_remaining);
            recursive_values.push(self.max_release(new_state, known_results)?);
        }
        Ok(recursive_values)
    }

    /*
     * One optimization that we may want to address is the case that we've evaluated (or started to evaluate)
     * a state like ("AA", {}, 30) and later find ourselves being asked to evaluate a state like
     * ("AA", {}, 28), i.e., the same starting node and set of closed valves, but with less time. It's
     * definitely true that we can skip trying to evaluate the latter, because we just looped back
     * around to where we'd come from without actually improving anything (i.e., without opening
     * any new valves). The trick is that I'm not sure how we'd actually _track_ that unless we
     * pass a vector of all the states we've passed through. It's not entirely clear how important
     * an optimization this would be, so I'm not quite sure what we do about it.
     */
    fn max_release(
        &self,
        state: State,
        known_results: &mut HashMap<State, u32>,
    ) -> anyhow::Result<u32> {
        println!("Current state: {state}");
        if state.time_remaining == 0 {
            return Ok(0);
        }
        let result = known_results.get(&state);
        if let Some(r) = result {
            return Ok(*r);
        }
        let time_remaining = state.time_remaining - 1;
        let mut open_valves = state.open_valves.clone();
        let mut my_values = self.process_name(
            &state.my_valve_name,
            &state.elephant_valve_name,
            time_remaining,
            &mut open_valves,
            known_results,
        )?;
        let elephant_values = self.process_name(
            &state.elephant_valve_name,
            &state.my_valve_name,
            time_remaining,
            &mut open_valves,
            known_results,
        )?;
        my_values.extend(elephant_values.into_iter());
        let result = my_values
            .into_iter()
            .max()
            .with_context(|| format!("There were no recursive results for state {state:?}"))?;
        known_results.insert(state, result);
        Ok(result)
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

    let cave = Cave::new(valves);

    let result = cave.max_release(
        State::new("AA", "AA", BitSet::default(), 26),
        &mut HashMap::new(),
    )?;

    println!("The maximum release is {result}");

    Ok(())
}
