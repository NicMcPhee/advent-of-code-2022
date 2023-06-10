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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
        result
            .into_iter()
            .flatten()
            // MizardX@Twitch pointed out that these "< 20" checks could be
            // moved into the `Add`/`Sub` implementations.
            .filter(|c| c.x < 20 && c.y < 20 && c.z < 20)
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

#[derive(Default, Debug)]
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
        self.num_cubes() * 6 - self.num_hidden_faces() //  - 6 * self.num_trapped_air_cubes()
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

    // fn num_trapped_air_cubes(&self) -> usize {
    //     todo!()
    // }

    fn find_outside(&mut self) {
        // `open` is a collection of all the cubes in the 20x20x20 volume
        // that are reachable "by water" from outside that volume. It starts
        // with all the cubes along the faces that are not rock (i.e., can be
        // filled with water).
        let mut open: Vec<Cube> = Vec::new();
        for u in 0u8..20 {
            for v in 0u8..20 {
                for s in [0u8, 19] {
                    for cube in [
                        Cube { x: u, y: v, z: s },
                        Cube { x: u, y: s, z: v },
                        Cube { x: s, y: u, z: v },
                    ] {
                        if self.is_not_rock(cube) {
                            open.push(cube);
                        }
                    }
                }
            }
        }

        while let Some(cube) = open.pop() {
            if self.outside_cubes.insert(cube) {
                for n in cube.neighbors() {
                    if self.is_not_rock(n) {
                        open.push(n);
                    }
                }
            }
        }
    }

    fn is_not_rock(&self, cube: Cube) -> bool {
        self.cubes.contains(&cube).not()
    }

    fn air_pocket_cubes(&self) -> Vec<Cube> {
        let mut result = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                for z in 0..20 {
                    let cube = Cube { x, y, z };
                    if (self.cubes.contains(&cube) || self.outside_cubes.contains(&cube)).not() {
                        result.push(cube);
                    }
                }
            }
        }
        result
    }
}

impl FromIterator<Cube> for LavaDroplet {
    fn from_iter<T: IntoIterator<Item = Cube>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), Self::add_cube)
    }
}

static INPUT_FILE: &str = "../inputs/day_18.input";

fn main() -> anyhow::Result<()> {
    let mut lava_droplet: LavaDroplet = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<LavaDroplet, _>>()?;

    lava_droplet.find_outside();
    let air_pocket_cubes: Vec<Cube> = lava_droplet.air_pocket_cubes();
    let air_pocket: LavaDroplet = air_pocket_cubes.into_iter().collect::<LavaDroplet>();

    println!("The air pocket was {air_pocket:?}");

    println!(
        "The surface area of the lava droplet is {}",
        lava_droplet.surface_area() - air_pocket.surface_area()
    );

    Ok(())
}
