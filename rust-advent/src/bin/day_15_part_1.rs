#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;
use regex::{Captures, Regex};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor(Point);
#[derive(Debug)]
struct Beacon(Point);

#[derive(Default)]
struct Cave {
    sensors: Vec<Sensor>,
    beacons: Vec<Beacon>,
}

impl Cave {
    fn add_entry(&mut self, capture: &Captures) -> anyhow::Result<()> {
        let sensor = Sensor(Point {
            x: capture[1].parse()?,
            y: capture[2].parse()?,
        });
        let beacon = Beacon(Point {
            x: capture[3].parse()?,
            y: capture[4].parse()?,
        });

        self.sensors.push(sensor);
        self.beacons.push(beacon);

        Ok(())
    }
}

static INPUT_FILE: &str = "../inputs/day_15_test.input";

fn main() -> anyhow::Result<()> {
    let mut cave = Cave::default();

    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    for cap in re.captures_iter(&contents) {
        cave.add_entry(&cap)?;
    }

    println!("Sensors: {:?}", cave.sensors);
    println!("Beacons: {:?}", cave.beacons);

    Ok(())
}
