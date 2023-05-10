#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashMap, fmt::Display, fs, ops::Not};

use anyhow::Context;
use itertools::Itertools;
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

    fn len(&self) -> usize {
        (0..64).filter(|i| self.contains(*i)).count()
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

#[derive(Clone)]
enum Move {
    Open(String),
    Move(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn moves(&self, valve_name: &str, open_valves: &BitSet) -> anyhow::Result<Vec<Move>> {
        let valve = self.numbered_valve(valve_name)?;
        let adjacent_valves = &valve.valve.adjacent_valve_names;
        let mut moves = Vec::with_capacity(1 + adjacent_valves.len());
        if valve.valve.flow_rate > 0 && open_valves.contains(valve.number).not() {
            moves.push(Move::Open(valve_name.to_string()));
        }
        moves.extend(adjacent_valves.iter().map(|v| Move::Move(v.clone())));
        Ok(moves)
    }

    fn apply_moves(
        &self,
        state: &State,
        my_move: &Move,
        elephant_move: &Move,
    ) -> anyhow::Result<(State, u32)> {
        let time_remaining = state.time_remaining - 1;
        match (my_move, elephant_move) {
            (Move::Open(my_valve), Move::Open(elephant_valve)) if my_valve == elephant_valve => {
                let numbered_valve = self.numbered_valve(my_valve)?;
                Ok((
                    State::new(
                        &state.my_valve_name,
                        &state.elephant_valve_name,
                        state.open_valves.insert(numbered_valve.number),
                        time_remaining,
                    ),
                    numbered_valve.valve.flow_rate * u32::from(time_remaining),
                ))
            }
            (Move::Open(my_valve), Move::Open(elephant_valve)) => {
                let my_numbered_valve = self.numbered_valve(my_valve)?;
                let elephant_numbered_valve = self.numbered_valve(elephant_valve)?;
                Ok((
                    State::new(
                        &state.my_valve_name,
                        &state.elephant_valve_name,
                        state
                            .open_valves
                            .insert(my_numbered_valve.number)
                            .insert(elephant_numbered_valve.number),
                        time_remaining,
                    ),
                    my_numbered_valve.valve.flow_rate * u32::from(time_remaining)
                        + elephant_numbered_valve.valve.flow_rate * u32::from(time_remaining),
                ))
            }
            (Move::Open(my_open_valve), Move::Move(elephant_valve)) => {
                let my_numbered_open_valve = self.numbered_valve(my_open_valve)?;
                let mut my_valve = state.my_valve_name.clone();
                let mut elephant_valve = elephant_valve.clone();
                if my_valve > elephant_valve {
                    (my_valve, elephant_valve) = (elephant_valve, my_valve);
                }
                Ok((
                    State::new(
                        &my_valve,
                        &elephant_valve,
                        state.open_valves.insert(my_numbered_open_valve.number),
                        time_remaining,
                    ),
                    my_numbered_open_valve.valve.flow_rate * u32::from(time_remaining),
                ))
            }
            (Move::Move(my_valve), Move::Open(elephant_open_valve)) => {
                let elephant_numbered_open_valve = self.numbered_valve(elephant_open_valve)?;
                let mut my_valve = my_valve.clone();
                let mut elephant_valve = state.elephant_valve_name.clone();
                if my_valve > elephant_valve {
                    (my_valve, elephant_valve) = (elephant_valve, my_valve);
                }
                Ok((
                    State::new(
                        &my_valve,
                        &elephant_valve,
                        state
                            .open_valves
                            .insert(elephant_numbered_open_valve.number),
                        time_remaining,
                    ),
                    elephant_numbered_open_valve.valve.flow_rate * u32::from(time_remaining),
                ))
            }
            (Move::Move(my_valve), Move::Move(elephant_valve)) => {
                let mut my_valve = my_valve.clone();
                let mut elephant_valve = elephant_valve.clone();
                if my_valve > elephant_valve {
                    (my_valve, elephant_valve) = (elephant_valve, my_valve);
                }
                Ok((
                    State::new(
                        &my_valve,
                        &elephant_valve,
                        state.open_valves.clone(),
                        time_remaining,
                    ),
                    0,
                ))
            }
        }
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
        if state.time_remaining == 0 {
            return Ok(0);
        }
        if state.open_valves.len() == 15 {
            println!(
                "Current state: {state}; known results size = {}",
                known_results.len()
            );
            return Ok(0);
        }
        let result = known_results.get(&state);
        if let Some(r) = result {
            return Ok(*r);
        }
        if known_results.len() % 1_000_000 == 0 {
            println!(
                "Current state: {state}; known results size = {}",
                known_results.len()
            );
        }
        let open_valves = state.open_valves.clone();
        let my_moves = self.moves(&state.my_valve_name, &open_valves)?;
        let elephant_moves = self.moves(&state.elephant_valve_name, &open_valves)?;
        // TODO: Use `map_ok()` from the `itertools` crate to avoid all this `collect()`-then-back-to-`iter()`
        //   business.
        let state_flow_pairs = my_moves
            .into_iter()
            .cartesian_product(elephant_moves)
            .map(|(my_move, elephant_move)| self.apply_moves(&state, &my_move, &elephant_move))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let new_values = state_flow_pairs
            .into_iter()
            .unique_by(|(state, _)| state.clone())
            .map(|(state, flow)| self.max_release(state, known_results).map(|f| f + flow))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let result = new_values
            .into_iter()
            .max()
            .with_context(|| format!("There were no recursive results for state {state:?}"))?;
        known_results.insert(state, result);
        Ok(result)
    }
}

static INPUT_FILE: &str = "../inputs/day_16.input";

fn main() -> anyhow::Result<()> {
    let valves = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(extract_valve)
        .collect::<anyhow::Result<Vec<Valve>>>()?;

    // println!("{valves:?}");

    let cave = Cave::new(valves);

    let result = cave.max_release(
        State::new("AA", "AA", BitSet::default(), 26),
        &mut HashMap::new(),
    )?;

    println!("The maximum release is {result}");

    Ok(())
}
