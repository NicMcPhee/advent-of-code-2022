#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::{anyhow, Result, Context};

static INPUT_FILE: &str = "../inputs/day_01.input";

// TODO: Extract Parts 1 and 2 out with tests.
fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    // let elves = contents
    //     .split("\n\n")
    //     .try_fold(None, |max_so_far, elf| 
    //         cmp::max(max_so_far.unwrap(), process_elf(elf)?))

    let mut elves = contents
        .split("\n\n")
        .map(process_elf)
        .collect::<Result<Vec<usize>>>()?;

    let num_elves = elves.len();

    let (_, pivot, big) 
        = elves.select_nth_unstable(num_elves - 3);
    
    // TODO: Try to do an error checked version of these accesses.
    let largest = std::cmp::max(big[0], big[1]);

    let sum_of_big_three = *pivot + big[0] + big[1];

    println!("The maximum calories for an elf was {largest}");
    println!("The sum of the big three was {sum_of_big_three}");

    Ok(())
}

fn day_01_part_1(input_file: &str) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod day_01_test {
    use crate::INPUT_FILE;

    use super::day_01_part_1;

    #[test]
    fn part_1() {
        let result = day_01_part_1(INPUT_FILE).unwrap();
        assert_eq!(74394, result);
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
        .map(|s| 
            s.parse::<usize>()
             .with_context(|| format!("Failed to parse '{s}' to `usize`")) 
        )
        .sum::<Result<usize>>()
}
