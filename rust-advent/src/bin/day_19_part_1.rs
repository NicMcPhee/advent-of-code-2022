#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, marker::PhantomData, str::FromStr};

use anyhow::Context;

#[derive(Debug)]
struct Ore;
#[derive(Debug)]
struct Clay;
#[derive(Debug)]
struct Obsidian;
#[derive(Debug)]
struct Geode;

#[derive(Debug)]
struct Robot<T> {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: u8,
    phantom: PhantomData<T>,
}

impl Robot<Ore> {
    const fn new_ore(ore_cost: u8) -> Self {
        Self {
            ore_cost,
            clay_cost: 0,
            obsidian_cost: 0,
            phantom: PhantomData,
        }
    }
}

impl Robot<Clay> {
    const fn new_clay(ore_cost: u8) -> Self {
        Self {
            ore_cost,
            clay_cost: 0,
            obsidian_cost: 0,
            phantom: PhantomData,
        }
    }
}

impl Robot<Obsidian> {
    const fn new_obsidian(ore_cost: u8, clay_cost: u8) -> Self {
        Self {
            ore_cost,
            clay_cost,
            obsidian_cost: 0,
            phantom: PhantomData,
        }
    }
}

impl Robot<Geode> {
    const fn new_geode(ore_cost: u8, obsidian_cost: u8) -> Self {
        Self {
            ore_cost,
            clay_cost: 0,
            obsidian_cost,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    number: u8,
    ore: Robot<Ore>,
    clay: Robot<Clay>,
    obsidian: Robot<Obsidian>,
    geode: Robot<Geode>,
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.

/*
let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "2012-03-14, 2013-01-01 and 2014-07-05";
for cap in re.captures_iter(text) {
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
}
 */

#[allow(clippy::expect_used)]
static IS_NUMBER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+)").expect("Failed to build the `IS_NUMBER` regex"));

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let numbers: Vec<u8> = IS_NUMBER
            .find_iter(s)
            .map(|m| m.as_str().parse::<u8>())
            .collect::<Result<_, _>>()?;
        Ok(Self {
            number: numbers[0],
            ore: Robot::new_ore(numbers[1]),
            clay: Robot::new_clay(numbers[2]),
            obsidian: Robot::new_obsidian(numbers[3], numbers[4]),
            geode: Robot::new_geode(numbers[5], numbers[6]),
        })
    }
}

static INPUT_FILE: &str = "../inputs/day_19.input";

fn main() -> anyhow::Result<()> {
    let blueprints: Vec<Blueprint> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Blueprint>, _>>()?;

    println!("The blueprints are {blueprints:?}");

    Ok(())
}
