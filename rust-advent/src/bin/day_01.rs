use std::fs;
use std::str::SplitAsciiWhitespace;

fn main() {
    let contents = fs::read_to_string("../inputs/day_01.input")
        .expect("Should have been able to read the file");

    let mut totals: Vec<usize> = contents
        .split("\n\n")
        .map(|g| g.split_ascii_whitespace())
        .map(sum_group)
        .collect();

    let biggest = totals.iter().max().unwrap();

    println!("The largest sum was {biggest}");

    totals.sort();
    let biggest_three: usize = totals.iter().rev().take(3).sum();

    println!("The sum of the three largest values was {biggest_three}.");
}

fn sum_group(group: SplitAsciiWhitespace) -> usize {
    group.map(|s| s.parse::<usize>().unwrap()).sum()
}
