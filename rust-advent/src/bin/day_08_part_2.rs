#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    iter::{repeat, Repeat, Rev, Zip},
    ops::Range,
    str::FromStr,
};

use anyhow::{Context, Result};
use rayon::prelude::{ParallelBridge, ParallelIterator};
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

#[derive(EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TODO: Is there a way to not have such specific types in
//   the parameters for these variants? I generally really like
//   this approach, but these types seem super specific to me
//   in a way that seems potentially fragile.
enum ForestIterator<UP, DOWN, LEFT, RIGHT> {
    Up(UP),
    Down(DOWN),
    Left(LEFT),
    Right(RIGHT),
}

impl<UP, DOWN, LEFT, RIGHT, I> Iterator for ForestIterator<UP, DOWN, LEFT, RIGHT>
where
    UP: Iterator<Item = I>,
    DOWN: Iterator<Item = I>,
    LEFT: Iterator<Item = I>,
    RIGHT: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Up(iter) => iter.next(),
            Self::Down(iter) => iter.next(),
            Self::Left(iter) => iter.next(),
            Self::Right(iter) => iter.next(),
        }
    }
}

type Up = Zip<Repeat<usize>, Rev<Range<usize>>>;
type Down = Zip<Repeat<usize>, Range<usize>>;
type Left = Zip<Rev<Range<usize>>, Repeat<usize>>;
type Right = Zip<Range<usize>, Repeat<usize>>;
impl ForestIterator<Up, Down, Left, Right> {
    // Should this be called `new()` instead?
    fn neighbors(direction: Direction, row: usize, col: usize, size: usize) -> Self {
        match direction {
            Direction::Up => Self::Up(repeat(row).zip((0..col).rev())),
            Direction::Down => Self::Down(repeat(row).zip(col + 1..size)),
            Direction::Left => Self::Left((0..row).rev().zip(repeat(col))),
            Direction::Right => Self::Right((row + 1..size).zip(repeat(col))),
        }
    }
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

    fn scenic_score_from(&self, row: usize, col: usize, direction: Direction) -> usize {
        let this_height = self.trees[row][col].height;
        let mut neighbors = ForestIterator::neighbors(direction, row, col, self.size());
        let count = neighbors
            .by_ref()
            .take_while(|(other_row, other_col)| {
                self.trees[*other_row][*other_col].height < this_height
            })
            .count();
        count + usize::from(neighbors.next().is_some())
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        Direction::iter()
            .map(|direction| self.scenic_score_from(row, col, direction))
            .product()
    }
}

fn max_scenic_score(contents: &str) -> Result<usize> {
    let forest = contents.parse::<Forest>()?;

    let size = forest.size();

    (0..size)
        .flat_map(|col_num| (0..size).zip(repeat(col_num)))
        .par_bridge()
        .map(|(row_num, col_num)| forest.scenic_score(row_num, col_num))
        .max()
        .context("max() was called on an empty list")
}

static INPUT_FILE: &str = "../inputs/day_08.input";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let max_score = max_scenic_score(&contents)?;

    println!("The maximum scenic score was {max_score}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input() {
        let s = "30373\n25512\n65332\n33549\n35390";
        let num_visible = max_scenic_score(s).unwrap();
        assert_eq!(21, num_visible);
    }
}
