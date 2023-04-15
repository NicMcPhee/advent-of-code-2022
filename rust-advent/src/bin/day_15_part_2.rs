#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{fs, ops::RangeInclusive};

use anyhow::Context;
use itertools::Itertools;
use range_union_find::IntRangeUnionFind;
use regex::{Captures, Regex};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor(Point);
#[derive(Debug)]
struct Beacon(Point);

#[derive(Debug)]
struct SensorBeacon {
    sensor: Sensor,
    beacon: Beacon,
    manhattan_distance: u32,
}

impl SensorBeacon {
    const fn new(sensor: Sensor, beacon: Beacon) -> Self {
        Self {
            manhattan_distance: Self::md(sensor.0, beacon.0),
            sensor,
            beacon,
        }
    }

    // TODO: Add a comment that explains the math here. Probably want to
    //   change some of the variables at the same time so the whole thing
    //   makes a little more sense.
    #[allow(clippy::cast_possible_wrap)]
    fn row_range(&self, row: i32) -> RangeInclusive<i32> {
        let sensor_row_dist = self.sensor.0.y.abs_diff(row);
        if sensor_row_dist > self.manhattan_distance {
            // If the sensor_row_dist is larger than the Manhattan distance
            // then the row is entirely outside the coverage area for the
            // sensor, and we want to return an empty range.
            #[allow(clippy::reversed_empty_ranges)]
            return 1..=0;
        }
        let ub: i32 = self.manhattan_distance as i32 - sensor_row_dist as i32;
        let lhs = self.sensor.0.x - ub;
        let rhs = self.sensor.0.x + ub;
        lhs.min(rhs)..=lhs.max(rhs)
    }

    const fn md(sensor: Point, beacon: Point) -> u32 {
        sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y)
    }
}

#[derive(Default)]
struct Cave {
    sensor_beacons: Vec<SensorBeacon>,
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
        let sensor_beacon = SensorBeacon::new(sensor, beacon);

        self.sensor_beacons.push(sensor_beacon);

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn coverage(&self, row: i32) -> anyhow::Result<u32> {
        // Replace the `for` loop with a fold or reduce?
        let mut union_range = IntRangeUnionFind::new();
        for r in self.row_ranges(row) {
            if !r.is_empty() {
                union_range.insert_range(&r)
                    .with_context(|| format!("Adding {r:?} to {union_range:?} failed"))?;
            }
        }
        println!("{union_range:?}");

        // The subtraction just before `.sum()` can never return a negative
        // value because the `end` of a range can't be less than the `start`
        // a range. (Maybe this is only partly true, but we'll hope that
        // no one is putting weird ranges into the system.)
        #[allow(clippy::cast_sign_loss)]
        let initial_count: u32 = union_range
            .to_collection::<Vec<_>>()
            .iter()
            .map(|r| (r.end() - r.start() + 1) as u32)
            .sum();

        println!("The initial count is {initial_count}.");

        let num_beacons_in_row = self
            .sensor_beacons
            .iter()
            .map(|sb| sb.beacon.0)
            .unique()
            .map(|pt| pt.y)
            .filter(|y| *y == row && union_range.has_element(y))
            .count() as u32;

        println!("The number of beacons in the range was {num_beacons_in_row}.");

        Ok(initial_count - num_beacons_in_row)
    }

    fn row_ranges(&self, row: i32) -> Vec<RangeInclusive<i32>> {
        self.sensor_beacons
            .iter()
            .map(|sensor_beacon| sensor_beacon.row_range(row))
            .collect()
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

    // println!("Sensors: {:?}", cave.sensor_beacons);

    println!("The row ranges are {:?}", cave.row_ranges(10));

    for row in 0..=20 {
        let num_covered = cave.coverage(row)?; // 2_000_000)?;
        println!("The number of covered locations = {num_covered}");
    }

    Ok(())
}
