#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    iter::{repeat, Repeat, Zip},
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
enum ForestIterator<A, B> {
    RowIter(A),    // Zip<Range<usize>, Repeat<usize>>),
    ColumnIter(B), // Zip<Repeat<usize>, Range<usize>>),
}

impl<A, B, C> Iterator for ForestIterator<A, B>
where
    A: Iterator<Item = C>,
    B: Iterator<Item = C>,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::RowIter(iter) => iter.next(),
            Self::ColumnIter(iter) => iter.next(),
        }
    }
}

type RowIterator = Zip<Range<usize>, Repeat<usize>>;
type ColumnIterator = Zip<Repeat<usize>, Range<usize>>;
impl ForestIterator<RowIterator, ColumnIterator> {
    // Should this be called `new()` instead?
    fn neighbors(direction: Direction, row: usize, col: usize, size: usize) -> Self {
        match direction {
            Direction::Up => Self::ColumnIter(repeat(row).zip(0..col)),
            Direction::Down => Self::ColumnIter(repeat(row).zip(col + 1..size)),
            Direction::Left => Self::RowIter((0..row).zip(repeat(col))),
            Direction::Right => Self::RowIter((row + 1..size).zip(repeat(col))),
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

    fn is_visible_from(&self, row: usize, col: usize, direction: Direction) -> bool {
        let this_height = self.trees[row][col].height;
        let mut neighbors = ForestIterator::neighbors(direction, row, col, self.size());
        neighbors
            .all(|(other_row, other_col)| this_height > self.trees[other_row][other_col].height)
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        Direction::iter().any(|direction| self.is_visible_from(row, col, direction))
    }
}

fn count_visible(contents: &str) -> Result<usize> {
    let forest = contents.parse::<Forest>()?;

    let size = forest.size();

    let num_visible_trees = (0..size)
        .flat_map(|col_num| (0..size).zip(repeat(col_num)))
        .par_bridge()
        .filter_map(|(row_num, col_num)| forest.is_visible(row_num, col_num).then_some(true))
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
        let num_visible = count_visible(s).unwrap();
        assert_eq!(21, num_visible);
    }
}
