#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    collections::HashSet,
    fs,
    ops::{Add, Not, Sub},
    str::FromStr,
};

use anyhow::Context;

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
    outside_cubes: HashSet<Cube>,
}

impl LavaDroplet {
    const fn num_hidden_faces(&self) -> usize {
        self.hidden_faces
    }

    fn num_cubes(&self) -> usize {
        self.cubes.len()
    }

    fn surface_area(&self) -> usize {
        self.num_cubes() * 6 - self.num_hidden_faces() - 6 * self.num_trapped_air_cubes()
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

    fn num_trapped_air_cubes(&self) -> usize {
        todo!()
    }

    fn find_outside(&self) {
        let mut open: Vec<Cube> = Vec::new();
        for x in 0u8..20 {
            for y in 0u8..20 {
                let cube = Cube { x, y, z: 0 };
                if self.is_outside(cube) {
                    open.push(cube);
                }
                let cube = Cube { x, y, z: 19 };
                if self.is_outside(cube) {
                    open.push(cube);
                }
            }
        }
        for x in 0u8..20 {
            for z in 0u8..20 {
                let cube = Cube { x, y: 0, z };
                if self.is_outside(cube) {
                    open.push(cube);
                }
                let cube = Cube { x, y: 19, z };
                if self.is_outside(cube) {
                    open.push(cube);
                }
            }
        }
        for y in 0u8..20 {
            for z in 0u8..20 {
                let cube = Cube { x: 0, y, z };
                if self.is_outside(cube) {
                    open.push(cube);
                }
                let cube = Cube { x: 19, y, z };
                if self.is_outside(cube) {
                    open.push(cube);
                }
            }
        }
        // Now we need loop until the open list is empty, taking
        // one cube from the open list, finding its neighbors,
        // and putting any non-rock neighbors back on the open list.
        // We also need to add the cube we took the list to our
        // `outside_cubes` set.
        todo!()
    }

    fn is_outside(&self, cube: Cube) -> bool {
        self.cubes.contains(&cube).not()
    }
}

impl FromIterator<Cube> for LavaDroplet {
    fn from_iter<T: IntoIterator<Item = Cube>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), Self::add_cube)
    }
}

static INPUT_FILE: &str = "../inputs/day_18_test.input";

fn main() -> anyhow::Result<()> {
    let lava_droplet: LavaDroplet = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<LavaDroplet, _>>()?;

    lava_droplet.find_outside();

    println!(
        "The surface area of the lava droplet is {}",
        lava_droplet.surface_area()
    );

    Ok(())
}
