#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{Context, Result};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::{
    fs::{self},
    iter::repeat,
    str::FromStr,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct Tree {
    height: u8,
}

impl Tree {
    const fn new(height: char) -> Self {
        Self {
            height: (height as u8) - b'0',
        }
    }
}

// This assumes we have a square array, but that appears to be true
// given the input.
// NathanielBumppo@Twitch suggested using `include_str!` as a possibility.
// This will read the file at _compile_ so the file contents are constant
// and we can use the calculated dimensions as the size of const generic
// arrays.
#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let trees = s
            .lines()
            .map(|s| s.chars().map(Tree::new).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let size = trees.len();

        // Check that the forest is square, sort of. All this
        // actually checks is that the number of rows is the same
        // as the number of columns in the first row, but that's
        // probably sufficient for now.
        anyhow::ensure!(size == trees[0].len());

        Ok(Self { trees })
    }
}

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    fn size(&self) -> usize {
        self.trees.len()
    }

    // 30373
    // 25512
    // 65332
    // 33549
    // 35390

    fn is_taller_than(
        &self,
        row: usize,
        col: usize,
        mut tree_locations: impl Iterator<Item = (usize, usize)>,
    ) -> bool {
        let this_height = self.trees[row][col].height;
        tree_locations
            .all(|(other_row, other_col)| this_height > self.trees[other_row][other_col].height)
    }

    fn is_visible_from(&self, row: usize, col: usize, direction: &Direction) -> bool {
        match direction {
            Direction::Up => self.is_taller_than(row, col, repeat(row).zip(0..col)),
            Direction::Down => self.is_taller_than(row, col, repeat(row).zip(col + 1..self.size())),
            Direction::Left => self.is_taller_than(row, col, (0..row).zip(repeat(col))),
            Direction::Right => {
                self.is_taller_than(row, col, (row + 1..self.size()).zip(repeat(col)))
            }
        }

        // let mut neighbors = match direction {
        //     Direction::Up => (0..col).map(Box::new(|col_num| (row, col_num)) as Box<dyn Fn (usize) -> (usize, usize)>),
        //     Direction::Down => (col+1..self.size()).map(Box::new(|col_num| (row, col_num)) as Box<dyn Fn (usize) -> (usize, usize)>),
        //     Direction::Left => (0..row).map(Box::new(|row_num| (row_num, col)) as Box<dyn Fn (usize) -> (usize, usize)>),
        //     Direction::Right => (row+1..self.size()).map(Box::new(|row_num| (row_num, col)) as Box<dyn Fn (usize) -> (usize, usize)>),
        // };
        // let this_height = self.trees[row][col].height;
        // neighbors.all(|(other_row, other_col)| this_height > self.trees[other_row][other_col].height)
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        Direction::iter().any(|direction| self.is_visible_from(row, col, &direction))
    }
}

fn count_visible(contents: &str) -> Result<usize> {
    let forest = contents.parse::<Forest>()?;

    let size = forest.size();

    let num_visible_trees = (0..size)
        .flat_map(|col_num| (0..size).zip(repeat(col_num)))
        .par_bridge()
        .filter(|(row_num, col_num)| forest.is_visible(*row_num, *col_num))
        .count();

    Ok(num_visible_trees)
}

static INPUT_FILE: &str = "../inputs/day_08.input";

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
        #[allow(clippy::unwrap_used)]
        let num_visible = count_visible(s).unwrap();
        assert_eq!(21, num_visible);
    }
}
