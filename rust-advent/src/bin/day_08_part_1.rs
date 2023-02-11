#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    cmp::max,
    fs::{self},
    str::FromStr,
};

use anyhow::{Context, Result};

// This is going to keep track of the tallest tree
// in each direction from a given position in the forest.
#[derive(Default, Clone, Debug)]
struct LargestNeighbors {
    left: Option<char>,
    right: Option<char>,
    up: Option<char>,
    down: Option<char>,
}

// This assumes we have a square array, but that appears to be true
// given the input.
// NathanielBumppo@Twitch suggested using `include_str!` as a possibility.
// This will read the file at _compile_ so the file contents are constant
// and we can use the calculated dimensions as the size of const generic
// arrays.
#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<char>>,
    largest_neighbors: Vec<Vec<LargestNeighbors>>,
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let trees = s
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let size = trees.len();

        anyhow::ensure!(size == trees[0].len());

        let largest_neighbors = vec![vec![LargestNeighbors::default(); size]; size];

        Ok(Self {
            trees,
            largest_neighbors,
        })
    }
}

impl Forest {
    // '/' is immediately before '0' in the ASCII/Unicode table so we
    // know it is less than any digit character.
    const MIN_HEIGHT: char = '/';
    // This almost works, but `char::from_u32` isn't stable as a const fn yet.
    // This should actually work in the current version of rust (1.67), but I'm
    // behind (1.65).
    // const min_height: char = {
    //     let zero_ord: u32 = '0' as u32;
    //     let min_char = char::from_u32(zero_ord).unwrap();
    //     min_char
    // };

    fn size(&self) -> usize {
        self.trees.len()
    }

    fn visible(&mut self, row: usize, col: usize) -> Result<bool> {
        let smallest_largest_neighbor = [
            self.largest_neighbors_left(row, col),
            self.largest_neighbors_right(row, col),
            self.largest_neighbors_up(row, col),
            self.largest_neighbors_down(row, col),
        ]
        .iter()
        .min()
        .copied()
        .with_context(|| {
            "There were no neighbors for the `min()` call at position ({row}, {col})"
        })?;
        Ok(self.trees[row][col] > smallest_largest_neighbor)
    }

    fn largest_neighbors_left(&mut self, row: usize, col: usize) -> char {
        if let Some(n) = self.largest_neighbors[row][col].left {
            return n;
        }
        if col == 0 {
            return Self::MIN_HEIGHT;
        }
        let lnll = self.largest_neighbors_left(row, col - 1);
        let l = self.trees[row][col - 1];
        let m = max(lnll, l);
        self.largest_neighbors[row][col].left = Some(m);

        m
    }

    fn largest_neighbors_right(&mut self, row: usize, col: usize) -> char {
        if let Some(n) = self.largest_neighbors[row][col].right {
            return n;
        }
        if col == self.size() - 1 {
            return Self::MIN_HEIGHT;
        }
        let lnrr = self.largest_neighbors_right(row, col + 1);
        let r = self.trees[row][col + 1];
        let m = max(lnrr, r);
        self.largest_neighbors[row][col].right = Some(m);

        m
    }

    fn largest_neighbors_up(&mut self, row: usize, col: usize) -> char {
        if let Some(n) = self.largest_neighbors[row][col].up {
            return n;
        }
        if row == 0 {
            return Self::MIN_HEIGHT;
        }
        let lnuu = self.largest_neighbors_up(row - 1, col);
        let u = self.trees[row - 1][col];
        let m = max(lnuu, u);
        self.largest_neighbors[row][col].up = Some(m);

        m
    }

    fn largest_neighbors_down(&mut self, row: usize, col: usize) -> char {
        if let Some(n) = self.largest_neighbors[row][col].down {
            return n;
        }
        if row == self.size() - 1 {
            return Self::MIN_HEIGHT;
        }
        let lndd = self.largest_neighbors_down(row + 1, col);
        let d = self.trees[row + 1][col];
        let m = max(lndd, d);
        self.largest_neighbors[row][col].down = Some(m);

        m
    }
}

static INPUT_FILE: &str = "../inputs/day_08.input";

fn count_visible(contents: &str) -> Result<usize> {
    let mut forest = contents.parse::<Forest>()?;

    // I do not like this looping and might want to come back to see
    // if I can avoid it.
    let mut num_visible_trees = 0;
    for row in 0..forest.size() {
        for col in 0..forest.size() {
            if forest.visible(row, col)? {
                num_visible_trees += 1;
                // println!("({row}, {col}) was visible");
            }
        }
    }
    // println!("{forest:#?}");

    Ok(num_visible_trees)
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let num_visible_trees = count_visible(&contents)?;

    println!("The number of visible trees was {num_visible_trees}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input() {
        let s = "30373\n25512\n65332\n33549\n35390";
        let num_visible = count_visible(s).unwrap();
        assert_eq!(21, num_visible);
    }
}
