use std::collections::HashSet;
use std::fs;

struct Rucksack {
    top: HashSet<char>,
    bottom: HashSet<char>,
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
        Rucksack {
            top: front_set,
            bottom: back_set,
        }
    }
}

impl Rucksack {
    fn shared_char(&self) -> char {
        *self.top.intersection(&self.bottom).next().unwrap()
    }

    fn all_elements(&self) -> HashSet<char> {
        self.top.union(&self.bottom).copied().collect()
    }

    fn intersect(&self, other: &Rucksack) -> HashSet<char> {
        self.all_elements()
            .intersection(&other.all_elements())
            .copied()
            .collect()
    }

    fn common_element(rucksacks: &[Rucksack]) -> char {
        assert!(rucksacks.len() == 3);
        *rucksacks[0]
            .intersect(&rucksacks[1])
            .intersection(&rucksacks[2].all_elements())
            .next()
            .unwrap()
    }
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        1 + c as u32 - ('a' as u32)
    } else if c.is_uppercase() {
        1 + 26 + (c as u32) - ('A' as u32)
    } else {
        panic!("The character {c} didn't work");
    }
}

fn main() {
    let contents = fs::read_to_string("../inputs/day_03.input")
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let rucksacks = lines.map(Rucksack::from);

    let total: u32 = rucksacks
        .clone()
        .map(|r| r.shared_char())
        .map(priority)
        .sum();

    println!("The total sum of priorities is {total}.");

    let group_total: u32 = rucksacks
        .collect::<Vec<_>>()
        .chunks(3)
        .map(Rucksack::common_element)
        .map(priority)
        .sum();

    println!("The total sum of group badge priorities is {group_total}.");
}
