#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Element {
    value: i64,
    initial_position: usize,
}

#[derive(Debug)]
struct MovedElement {
    current_position: usize,
    new_position: usize,
}

impl MovedElement {
    fn new(initial_position: usize, values: &[Element]) -> anyhow::Result<Self> {
        let current_position = values
            .iter()
            .position(|e| e.initial_position == initial_position)
            .with_context(|| {
                format!("Failed to find element with initial position {initial_position}.")
            })?;
        let element = values
            .get(current_position)
            .with_context(|| format!("Retrieving element at index {current_position} failed"))?;

        let length = values.len();
        let length_i64 = i64::try_from(length)?;
        let offset = usize::try_from(element.value.rem_euclid(length_i64 - 1))?;

        let new_position = if current_position + offset < length {
            current_position + offset
        } else {
            current_position + offset - (length - 1)
        };

        Ok(Self {
            current_position,
            new_position,
        })
    }
}

fn mix(values: &mut Vec<Element>) -> anyhow::Result<()> {
    for i in 0..values.len() {
        move_element(values, i)?;
        // let vals: Vec<i64> = values.iter().map(|e| e.value).collect();
        // println!("Current vals = {vals:?}");
    }
    Ok(())
}

/*
 * If value is non-negative and adding value and the position < length,
 *    then we rotate_left with the slice [pos..pos+val]
 * If value is negative and adding val and pos < 0,
 *    then we rotate_left with the slice [pos..pos+val+len]
 *
 * If value is non-negative and adding value and position >= length,
 *    then we rotate_right with the slice [pos+val-len..pos]
 * If value is negative and adding val and pos >= 0,
 *    then we rotate_right with the slice [pos+val..pos]
 *
 * Make sure handle value = 0.
 */
fn move_element(values: &mut [Element], i: usize) -> anyhow::Result<()> {
    let moved_element = MovedElement::new(i, values)?;

    let current_position = moved_element.current_position;
    let new_position = moved_element.new_position;

    match current_position.cmp(&new_position) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Less => values[current_position..=new_position].rotate_left(1),
        std::cmp::Ordering::Greater => values[new_position..=current_position].rotate_right(1),
    }
    Ok(())
}

fn compute_result(values: &Vec<Element>) -> anyhow::Result<i64> {
    let length = values.len();
    let zero_position = values
        .iter()
        .position(|e| e.value == 0)
        .with_context(|| "Failed to find element with value 0.")?;
    println!(
        "Position of zero is {zero_position}, with element {:?}",
        values[zero_position]
    );

    Ok([1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let i = (zero_position + offset) % length;
            println!(
                "Offset {offset} with position {i} and value {}",
                values[i].value
            );
            values[i].value
        })
        .sum())
}

static INPUT_FILE: &str = "../inputs/day_20.input";

fn main() -> anyhow::Result<()> {
    let mut values: Vec<Element> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse::<i64>)
        .enumerate()
        .map(|(i, r)| {
            Ok(Element {
                value: r? * 811_589_153,
                initial_position: i,
            })
        })
        .collect::<anyhow::Result<Vec<Element>>>()?;

    for _ in 0..10 {
        mix(&mut values)?;
    }

    println!("After mixing: {values:?}");

    let result = compute_result(&values);

    println!("The result is {result:?}");

    Ok(())
}
