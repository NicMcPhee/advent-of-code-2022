#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    cmp::max,
    fs::{self},
    slice::Iter,
    str::FromStr,
};

use anyhow::{bail, ensure, Context, Result};

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: Option<bool>,
}

impl Tree {
    fn new(height: char) -> Self {
        Self {
            height: (height as u8) - b'0',
            visible: None,
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
            .map(|s| s.chars().map(|c| Tree::new(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let size = trees.len();

        anyhow::ensure!(size == trees[0].len());

        Ok(Forest { trees })
    }
}

enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

struct ForestIter<'forest> {
    forest: &'forest mut Forest,
    direction: Direction,
    position: usize,
}

impl<'forest> Iterator for ForestIter<'forest>
where
    Self: 'forest,
{
    type Item = Box<dyn Iterator<Item = &'forest mut Tree> + 'forest>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.forest.size() {
            return None;
        }
        let position = self.position;
        match self.direction {
            Direction::LeftToRight => Some(Box::new(self.forest.trees[position].iter_mut())),
            _ => None,
            // Direction::RightToLeft => Some(Box::new(self.forest.trees[position].iter_mut().rev())),
            // Direction::TopToBottom => Some(Box::new(self.forest.trees.iter_mut().map(move |row| todo!()))),
            // Direction::BottomToTop => Some(Box::new(self.forest.trees.iter_mut().map(move |row| &mut row[position]).rev())),
        }
    }
}

impl<'forest> ForestIter<'forest> {
    fn new(forest: &'forest mut Forest, direction: Direction) -> Self {
        Self {
            forest,
            direction,
            position: 0,
        }
    }
}

impl Forest {
    fn size(&self) -> usize {
        self.trees.len()
    }

    fn visible(&self, row: usize, col: usize) -> Result<bool> {
        ensure!(row < self.size());
        ensure!(col < self.size());
        let result = self.trees[row][col].visible;
        match result {
            None => bail!("Called visible on a tree ({row}, {col}) that hadn't been processed"),
            Some(visible) => Ok(visible),
        }
    }

    fn process_slice(&self, slice_iterator: Box<dyn Iterator<Item = &mut Tree>>) {
        let tallest_so_far: i32 = -1;
        for tree in slice_iterator {
            let visible = tree.visible.unwrap() || tree.height as i32 > tallest_so_far;
            tree.visible = Some(true);
        }
        todo!();
    }

    // 30373
    // 25512
    // 65332
    // 33549
    // 35390

    fn visible_from_direction(&mut self, direction: Direction) {
        let forest_iterator = ForestIter::new(self, direction);
        forest_iterator.for_each(|slice_iterator| {
            self.process_slice(slice_iterator);
        });
    }

    fn compute_visibilities(&mut self) {
        for direction in [
            Direction::BottomToTop,
            Direction::LeftToRight,
            Direction::RightToLeft,
            Direction::TopToBottom,
        ] {
            self.visible_from_direction(direction);
        }
    }
}

static INPUT_FILE: &str = "../inputs/day_08.input";

fn count_visible(contents: &str) -> Result<usize> {
    let mut forest = contents.parse::<Forest>()?;

    forest.compute_visibilities();

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
