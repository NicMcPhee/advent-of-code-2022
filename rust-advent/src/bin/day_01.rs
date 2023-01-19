#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::{bail, Context, Result};

static INPUT_FILE: &str = "../inputs/day_01.input";

// TODO: Extract Parts 1 and 2 out with tests.
fn main() -> Result<()> {
    let big_three: [usize; 3] = process_elves(INPUT_FILE)?;

    println!("The big three are {big_three:?}");

    let largest = big_three
        .iter()
        .max()
        .context("The array of biggest elves was empty (which should be impossible)")?;

    let sum_of_big_three: usize = big_three.iter().sum();

    println!("The maximum calories for an elf was {largest}");
    println!("The sum of the big three was {sum_of_big_three}");

    Ok(())
}

fn process_elves(input_file: &str) -> Result<[usize; 3]> {
    let contents = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to open file '{input_file}'"))?;

    let mut elves = contents
        .split("\n\n")
        .map(process_elf)
        .collect::<Result<Vec<usize>>>()?;

    let num_elves = elves.len();

    if num_elves < 3 {
        bail!("We had fewer than three elves in the input file");
    }

    let (_, pivot, big) = elves.select_nth_unstable(num_elves - 3);

    let top_three = [*pivot, big[0], big[1]];

    Ok(top_three)
}

#[cfg(test)]
mod process_elves_test {
    use super::process_elves;
    use super::INPUT_FILE;

    #[test]
    fn check_process_elves() {
        let mut big_three = process_elves(INPUT_FILE).unwrap();
        big_three.sort();
        assert_eq!([68579, 69863, 74394], big_three);
    }
}

/**
 * An "elf" is a string that is a sequence of numbers, each
 * on their own line. E.g.,
 *
 * 10040
 * 9088
 * 11305
 * 5867
 * 10766
 * 9996
 * 11092
 *
 * Our goal is to split that into individual numbers, parse
 * them into `usize` values, and add them up.
 */
fn process_elf(elf_str: &str) -> Result<usize> {
    elf_str
        .split_ascii_whitespace()
        .map(|s| {
            s.parse::<usize>()
                .with_context(|| format!("Failed to parse '{s}' to `usize`"))
        })
        .sum::<Result<usize>>()
}
