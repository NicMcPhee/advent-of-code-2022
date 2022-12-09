use std::collections::HashSet;
use std::fs;

struct Rucksack {
    top: HashSet<char>,
    bottom: HashSet<char>
}

fn char_set(s: &str) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let compartment_size = s.len() / 2;
        let (front_chars, back_chars) = s.split_at(compartment_size);
        let front_set = char_set(front_chars);
        let back_set = char_set(back_chars);
        Rucksack { top:front_set, bottom: back_set }
    }
}

impl Rucksack {
    fn shared_char(&self) -> char {
        *self.top.intersection(&self.bottom).next().unwrap()
    }
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        return 1 + c as u32 - ('a' as u32);
    } else if c.is_uppercase() {
        return 1 + 26 + (c as u32) - ('A' as u32);
    } else {
        panic!("The character {c} didn't work");
    }
}

fn main() {
    let total: u32 = fs::read_to_string("../inputs/day_03.input")
        .expect("Should have been able to read the file")
        .lines()
        .map(Rucksack::from)
        .map(|r| r.shared_char())
        .map(priority)
        .sum();

    println!("The total sum of priorities is {total}.");

    // let mut totals: Vec<usize> = contents
    //     .split("\n\n")
    //     .map(|g| g.split_ascii_whitespace())
    //     .map(|g| sum_group(g))
    //     .collect();

    // let biggest = totals.iter().max().unwrap();

    // println!("The largest sum was {biggest}");

    // totals.sort();
    // let biggest_three: usize = totals.iter().rev().take(3).sum();

    // println!("The sum of the three largest values was {biggest_three}.");
}
