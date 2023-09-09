#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use std::{
    collections::HashSet,
    fs,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Add for Cube {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        Some(Self {
            x: self.x.checked_add(rhs.x)?,
            y: self.y.checked_add(rhs.y)?,
            z: self.z.checked_add(rhs.z)?,
        })
    }
}

impl Sub for Cube {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        Some(Self {
            x: self.x.checked_sub(rhs.x)?,
            y: self.y.checked_sub(rhs.y)?,
            z: self.z.checked_sub(rhs.z)?,
        })
    }
}

impl Cube {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        let result = [
            self + Self { x: 1, y: 0, z: 0 },
            self - Self { x: 1, y: 0, z: 0 },
            self + Self { x: 0, y: 1, z: 0 },
            self - Self { x: 0, y: 1, z: 0 },
            self + Self { x: 0, y: 0, z: 1 },
            self - Self { x: 0, y: 0, z: 1 },
        ];
        result.into_iter().flatten()
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let coordinates = s
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        anyhow::ensure!(coordinates.len() == 3);
        Ok(Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        })
    }
}

#[derive(Default)]
struct LavaDroplet {
    cubes: HashSet<Cube>,
    hidden_faces: usize,
}

impl LavaDroplet {
    const fn num_hidden_faces(&self) -> usize {
        self.hidden_faces
    }

    fn num_cubes(&self) -> usize {
        self.cubes.len()
    }

    fn surface_area(&self) -> usize {
        self.num_cubes() * 6 - self.num_hidden_faces()
    }

    fn add_cube(mut self, cube: Cube) -> Self {
        for neighbor in cube.neighbors() {
            if self.cubes.contains(&neighbor) {
                self.hidden_faces += 2;
            }
        }
        self.cubes.insert(cube);
        self
    }
}

impl FromIterator<Cube> for LavaDroplet {
    fn from_iter<T: IntoIterator<Item = Cube>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), Self::add_cube)
    }
}

static INPUT_FILE: &str = "../inputs/day_18.input";

fn main() -> anyhow::Result<()> {
    let lava_droplet: LavaDroplet = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<LavaDroplet, _>>()?;

    println!(
        "The surface area of the lava droplet is {}",
        lava_droplet.surface_area()
    );

    Ok(())
}
