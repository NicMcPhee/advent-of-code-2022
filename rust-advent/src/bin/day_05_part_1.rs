#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_05.input";

const NUM_STACKS: usize = 9;

#[derive(Default)]
struct Stacks {
    stacks: [Vec<char>; NUM_STACKS],
}

// An alternative approach would be to just do `line.chars()` and
// `line.nth(4)` for each stack in the mapping. That would avoid
// creating the `Vec<char>` for `line` that we have at the moment.
// We'd probably have to special case the first stack, though, and
// that would be ugly, so I'm not sure that wins.
fn extract_stack_elements(line: &str) -> Vec<char> {
    let line = line.chars().collect::<Vec<_>>();
    (0..NUM_STACKS).map(|pos| line[1 + 4 * pos]).collect()
}

#[cfg(test)]
mod extract_stack_elements_test {
    use super::*;

    #[test]
    fn extract() {
        let line = "[S] [J] [C]     [F] [C]     [D] [G]";
        let result = extract_stack_elements(line);
        assert_eq!(result, vec!['S', 'J', 'C', ' ', 'F', 'C', ' ', 'D', 'G']);
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    // TODO: Having a `fold()` inside a `fold()` here is pretty hard
    //   to think about, and I think I want to extract the inner fold into
    //   a named helper function. It seems that the same rules about
    //   not nesting for-loops probably applies to thing like nesting
    //   fold() calls as well.
    // TODO: I don't _really_ understand how the mutability rules are
    //   playing out here, and certainly expected that more instances of
    //   of `stacks` would need to be declared as `mut` than we
    //   ultimately needed. I think the trick is that we passed ownership
    //   of the set of stacks in to the `fold()` and inner `fold()`, so
    //   every closure that has a reference to `stacks` _owns_ that
    //   reference and can thus mutate it. If we held an incoming
    //   reference to `stacks` and referred to it later, then we
    //   would need to retain ownership and probably need a `mut`
    //   somewhere, but since we don't it doesn't present itself as
    //   as an issue.
    fn from_str(s: &str) -> Result<Self> {
        let stacks = s
            .lines()
            // We reverse the lines because we want the "bottom" lines
            // to be pushed onto the stacks first so those values end
            // up on the bottom of the stacks.
            .rev()
            // We'll just skip the line with the stack numbers since we
            // never use them.
            .skip(1)
            // Convert each line to a `Vec<char>` that holds the 9 elements
            // at each level. We'll put spaces in that `Vec<char>` for stacks
            // that don't have anything at that level.
            .map(extract_stack_elements)
            // "Loop" over each line/level, pushing the non-space values onto
            // the appropriate stacks.
            .fold(Self::default(), |stacks, line| {
                stacks.push_values_on_stacks(&line)
            });

        Ok(stacks)
    }
}

impl Stacks {
    // Note that the argument here is `self` and not `&self` because we
    // need to take ownership of this `Stacks` value so we can mutate
    // it in the `fold()` call. Alternatively we could declare this
    // as taking `&mut self`, but the calling function has no need
    // access the "old" value so there's really no need.
    fn push_values_on_stacks(self, line: &[char]) -> Self {
        line.iter()
            .enumerate()
            .filter(|&(_, c)| *c != ' ')
            .fold(self, |mut stacks, (i, c)| {
                stacks.stacks[i].push(*c);
                stacks
            })
    }
}

#[cfg(test)]
mod stacks_from_str_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        // TODO: This string is pretty ugly. @esitsu@Twitch suggested in `indoc`
        //   crate, which would allow us to indent things the way we want here,
        //   and it would left shift everything so it was up against the left
        //   side. That would be nicer, but I don't know if I'll take the time
        //   to do this?
        let input = "    [D]                                       
[N] [C]                                           
[Z] [M] [P]                                       
 1   2   3";
        let stacks: Stacks = input.parse().unwrap();
        assert_eq!(2, stacks.stacks[0].len());
        assert_eq!(3, stacks.stacks[1].len());
        assert_eq!(vec!['M', 'C', 'D'], stacks.stacks[1]);
        assert_eq!(1, stacks.stacks[2].len());
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (stack_config, instruction) = contents
        .split_once("\n\n")
        .context("There was no blank line in the input")?;

    let stacks: Stacks = stack_config.parse()?;

    // println!("The sum of the priorities is {sum_of_priorities:?}");

    Ok(())
}
