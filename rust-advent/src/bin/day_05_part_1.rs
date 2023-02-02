#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{fs, str::FromStr};

use anyhow::{ensure, Context, Result};

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

    fn apply(
        mut self,
        Instruction {
            num_to_move,
            from_stack,
            to_stack,
        }: Instruction,
    ) -> Result<Self> {
        for _ in 0..num_to_move {
            let value_to_move = self.stacks[from_stack - 1].pop().with_context(|| {
                format!("We tried to pop from stack {from_stack}, which was empty")
            })?;
            self.stacks[to_stack - 1].push(value_to_move);
        }
        Ok(self)
    }

    fn tops_string(&self) -> Result<String> {
        self.stacks
            .iter()
            .map(|s| {
                s.last().copied().with_context(|| {
                    format!("We tried to take the top of an empty stack: {:?}", self)
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
        let num_to_move = parts[0];
        let from_stack = parts[1];
        let to_stack = parts[2];
        Ok(Self {
            num_to_move,
            from_stack,
            to_stack,
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
