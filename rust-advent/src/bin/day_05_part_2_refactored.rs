#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{ensure, Context, Result};
use std::{fs, str::FromStr};

static INPUT_FILE: &str = "../inputs/day_05.input";

const NUM_STACKS: usize = 9;

#[derive(Default, Debug)]
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

impl FromStr for Stacks {
    type Err = anyhow::Error;

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

    // Two solutions:
    //   - One that allocates on the heap (using `collect()`)
    //   - One that avoids the additional heap allocation by using `split_at_mut()`
    //   - Discussion of pros and cons of each

    // - Give them the version that doesn't compile
    // - Have them explain why it doesn't work
    // - Have them fix it through re-ordering of existing lines, with explanation
    // - Have them fix it using `split_at_mut()` with explanation
    // - GitHub CoPilot solved the problem in a fairly reasonable way similar to
    //   the `get_two_mut()` solution, but without extracting it as a function.

    // Question? Should I remove the `collect()` call from the version they get?
    // If I do, they will need to move the `destination` declaration _and_ introduce
    // a `collect()` call in the first version.

    // The method `get_many_mut()` (https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get_many_mut)
    // would do this for us, but it's still in nightly. This will be renamed to `get_disjoint_mut` with stabilization.
    fn get_two_mut<T>(source: &mut [T], index1: usize, index2: usize) -> (&mut T, &mut T) {
        assert!(index1 != index2);
        if index1 < index2 {
            let (left, right) = source.split_at_mut(index2);
            (&mut left[index1], &mut right[0])
        } else {
            let (left, right) = source.split_at_mut(index1);
            (&mut right[0], &mut left[index2])
        }
    }

    fn apply(
        mut self,
        Instruction {
            num_to_move,
            from_stack,
            to_stack,
        }: Instruction,
    ) -> Result<Self> {
        // We know from the parsing check that `from_stack` and `to_stack` are different.
        let (source, destination) =
            Self::get_two_mut(&mut self.stacks, from_stack - 1, to_stack - 1);

        ensure!(
            num_to_move <= source.len(),
            "We tried to take {num_to_move} items from {source:?}"
        );

        let crates_to_move = source.drain((source.len() - num_to_move)..);
        destination.extend(crates_to_move);
        Ok(self)
    }

    fn tops_string(&self) -> Result<String> {
        self.stacks
            .iter()
            .map(|s| {
                s.last().copied().with_context(|| {
                    format!("We tried to take the top of an empty stack: {self:?}")
                })
            })
            .collect::<Result<String>>()
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
        #[allow(clippy::unwrap_used)]
        let stacks: Stacks = input.parse().unwrap();
        assert_eq!(2, stacks.stacks[0].len());
        assert_eq!(3, stacks.stacks[1].len());
        assert_eq!(vec!['M', 'C', 'D'], stacks.stacks[1]);
        assert_eq!(1, stacks.stacks[2].len());
    }
}

struct Instruction {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    // move 13 from 8 to 7
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<usize> = s
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(pos, _)| pos % 2 == 1)
            .map(|(_, val)| {
                val.parse::<usize>()
                    .with_context(|| format!("Couldn't parse '{val}' to an int"))
            })
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            parts.len() == 3,
            "Line '{s}' didn't have the appropriate format"
        );
        ensure!(
            parts[1] != parts[2],
            "The source {} and destination {} stacks must be different",
            parts[1],
            parts[2],
        );
        Ok(Self {
            num_to_move: parts[0],
            from_stack: parts[1],
            to_stack: parts[2],
        })
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (stack_config, instructions) = contents
        .split_once("\n\n")
        .context("There was no blank line in the input")?;

    let stacks: Stacks = stack_config.parse()?;

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    let final_state = instructions.into_iter().try_fold(stacks, Stacks::apply)?;

    println!("The top of the stacks is {}", final_state.tops_string()?);

    Ok(())
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
