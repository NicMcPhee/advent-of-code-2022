#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    fs,
    marker::PhantomData,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::Context;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Ore;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Clay;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Obsidian;
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Geode;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Blueprint {
    number: u8,
    ore: Robot<Ore>,
    clay: Robot<Clay>,
    obsidian: Robot<Obsidian>,
    geode: Robot<Geode>,
    max_ore_required: u8,
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.

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
        let max_ore_required = [numbers[1], numbers[2], numbers[3], numbers[5]]
            .into_iter()
            .max()
            .context("The set of ore requirements was somehow empty.")?;
        Ok(Self {
            number: numbers[0],
            ore: Robot::new_ore(numbers[1]),
            clay: Robot::new_clay(numbers[2]),
            obsidian: Robot::new_obsidian(numbers[3], numbers[4]),
            geode: Robot::new_geode(numbers[5], numbers[6]),
            max_ore_required,
        })
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
struct Resources {
    num_ore: u8,
    num_clay: u8,
    num_obsidian: u8,
    num_geodes: u8,
}

impl Resources {
    const fn from_robot<T>(robot: &Robot<T>) -> Self {
        Self {
            num_ore: robot.ore_cost,
            num_clay: robot.clay_cost,
            num_obsidian: robot.obsidian_cost,
            num_geodes: 0,
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        #[allow(clippy::expect_used)]
        Self {
            num_ore: self
                .num_ore
                .checked_add(rhs.num_ore)
                .expect("We overflowed the number of ore"),
            num_clay: self
                .num_clay
                .checked_add(rhs.num_clay)
                .expect("We overflowed the number of clay"),
            num_obsidian: self
                .num_obsidian
                .checked_add(rhs.num_obsidian)
                .expect("We overflowed the number of obsidian"),
            num_geodes: self
                .num_geodes
                .checked_add(rhs.num_geodes)
                .expect("We overflowed the number of geodes"),
        }
    }
}

impl Sub for Resources {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        Some(Self {
            num_ore: self.num_ore.checked_sub(rhs.num_ore)?,
            num_clay: self.num_clay.checked_sub(rhs.num_clay)?,
            num_obsidian: self.num_obsidian.checked_sub(rhs.num_obsidian)?,
            num_geodes: self.num_geodes.checked_sub(rhs.num_geodes)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct RobotCounts {
    num_ore_robots: u8,
    num_clay_robots: u8,
    num_obsidian_robots: u8,
    num_geode_robots: u8,
}

impl RobotCounts {
    const fn add_ore_robot(self) -> Self {
        Self {
            num_ore_robots: self.num_ore_robots + 1,
            ..self
        }
    }

    const fn add_clay_robot(self) -> Self {
        Self {
            num_clay_robots: self.num_clay_robots + 1,
            ..self
        }
    }

    const fn add_obsidian_robot(self) -> Self {
        Self {
            num_obsidian_robots: self.num_obsidian_robots + 1,
            ..self
        }
    }

    const fn add_geode_robot(self) -> Self {
        Self {
            num_geode_robots: self.num_geode_robots + 1,
            ..self
        }
    }
}

impl Default for RobotCounts {
    fn default() -> Self {
        Self {
            num_ore_robots: 1,
            num_clay_robots: 0,
            num_obsidian_robots: 0,
            num_geode_robots: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    remaining_minutes: u8,
    resources: Resources,
    robot_counts: RobotCounts,
}

impl Default for State {
    fn default() -> Self {
        Self {
            remaining_minutes: 32,
            resources: Resources::default(),
            robot_counts: RobotCounts::default(),
        }
    }
}

impl State {
    const fn new_resources(&self) -> Resources {
        Resources {
            num_ore: self.robot_counts.num_ore_robots,
            num_clay: self.robot_counts.num_clay_robots,
            num_obsidian: self.robot_counts.num_obsidian_robots,
            num_geodes: self.robot_counts.num_geode_robots,
        }
    }
}

impl Blueprint {
    fn quality_level(&self) -> usize {
        let state = State::default();
        usize::from(self.max_geodes(&state))
    }

    fn max_geodes(&self, state: &State) -> u8 {
        if state.remaining_minutes == 0 {
            // println!("{state:?}");
            return state.resources.num_geodes;
        }
        #[allow(clippy::expect_used)]
        self.child_states(state)
            .into_iter()
            .map(|s| self.max_geodes(&s))
            .max()
            .expect("There should have been at least one child state")
    }

    fn child_states(&self, state: &State) -> Vec<State> {
        // Make robots & collect resources
        if let Some(geode_state) = self.make_geode_robot(state) {
            return vec![geode_state];
        }
        // if let Some(obsidian_state) = self.make_obsidian_robot(state) {
        //     return vec![obsidian_state];
        // }
        // if let Some(clay_state) = self.make_clay_robot(state) {
        //     return vec![clay_state];
        // }
        // if let Some(ore_state) = self.make_ore_robot(state) {
        //     return vec![ore_state];
        // }
        // vec![self.make_no_robot(state).unwrap()]
        [
            Self::make_no_robot,
            Self::make_ore_robot,
            Self::make_clay_robot,
            Self::make_obsidian_robot,
            // Self::make_geode_robot,
        ]
        .into_iter()
        .filter_map(|f| f(self, state))
        .collect::<Vec<_>>()
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn make_no_robot(&self, state: &State) -> Option<State> {
        if state.robot_counts.num_ore_robots > self.max_ore_required
            || state.robot_counts.num_clay_robots > self.obsidian.clay_cost
            || state.resources.num_ore > self.max_ore_required
        // || state.resources.num_clay > self.obsidian.clay_cost
        {
            return None;
        }
        Some(State {
            remaining_minutes: state.remaining_minutes - 1,
            resources: state.resources + state.new_resources(),
            robot_counts: state.robot_counts,
        })
    }

    fn make_ore_robot(&self, state: &State) -> Option<State> {
        if state.robot_counts.num_ore_robots >= self.max_ore_required
            || state.resources.num_ore >= 8
        {
            return None;
        }
        Some(State {
            remaining_minutes: state.remaining_minutes - 1,
            resources: (state.resources - Self::robot_costs(&self.ore))? + state.new_resources(),
            robot_counts: state.robot_counts.add_ore_robot(),
        })
    }

    fn make_clay_robot(&self, state: &State) -> Option<State> {
        if state.robot_counts.num_clay_robots >= self.obsidian.clay_cost
            || state.resources.num_clay >= 30
        {
            return None;
        }
        Some(State {
            remaining_minutes: state.remaining_minutes - 1,
            resources: (state.resources - Self::robot_costs(&self.clay))? + state.new_resources(),
            robot_counts: state.robot_counts.add_clay_robot(),
        })
    }

    fn make_obsidian_robot(&self, state: &State) -> Option<State> {
        if state.robot_counts.num_obsidian_robots >= self.geode.obsidian_cost {
            return None;
        }
        Some(State {
            remaining_minutes: state.remaining_minutes - 1,
            resources: (state.resources - Self::robot_costs(&self.obsidian))?
                + state.new_resources(),
            robot_counts: state.robot_counts.add_obsidian_robot(),
        })
    }

    fn make_geode_robot(&self, state: &State) -> Option<State> {
        Some(State {
            remaining_minutes: state.remaining_minutes - 1,
            resources: (state.resources - Self::robot_costs(&self.geode))? + state.new_resources(),
            robot_counts: state.robot_counts.add_geode_robot(),
        })
    }

    const fn robot_costs<T>(robot: &Robot<T>) -> Resources {
        Resources::from_robot(robot)
    }
}

static INPUT_FILE: &str = "../inputs/day_19.input";

fn main() -> anyhow::Result<()> {
    let blueprints: Vec<Blueprint> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .take(3)
        .collect::<Result<Vec<Blueprint>, _>>()?;

    println!("The blueprints are {blueprints:?}");

    let result: usize = blueprints.iter().map(Blueprint::quality_level).product();

    println!("The result is {result}.");

    Ok(())
}
