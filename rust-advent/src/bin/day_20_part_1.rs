#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;

// Have a Vector of (value, initial_position).

#[derive(Debug, Copy, Clone)]
struct Element {
    value: i16,
    initial_position: usize,
}

#[derive(Debug)]
struct MovedElement {
    element: Element,
    current_position: usize,
    new_position: usize,
}

impl MovedElement {
    fn new(initial_position: usize, values: &[Element]) -> anyhow::Result<Self> {
        let length_i16 = i16::try_from(values.len())
            .with_context(|| format!("Converting length {} to i16 failed.", values.len()))?;

        let current_position = values
            .iter()
            .position(|e| e.initial_position == initial_position)
            .with_context(|| {
                format!("Failed to find element with initial position {initial_position}.")
            })?;
        let element = values
            .get(current_position)
            .with_context(|| format!("Retrieving element at index {current_position} failed"))?;
        let current_position_i16 = i16::try_from(current_position).with_context(|| {
            format!("Couldn't convert current position {current_position} to an `i16`")
        })?;

        let mut new_position_i16 = (current_position_i16 + element.value).rem_euclid(length_i16);
        if new_position_i16 == 0 && element.value < 0 {
            new_position_i16 += length_i16 - 1;
        }
        let new_position = usize::try_from(new_position_i16).with_context(|| {
            format!("Converting new position as i16 {new_position_i16} to usize failed.")
        })?;

        Ok(Self {
            element: *element,
            current_position,
            new_position,
        })
    }

    const fn value(&self) -> i16 {
        self.element.value
    }
}

fn mix(values: &mut Vec<Element>) -> anyhow::Result<()> {
    for i in 0..values.len() {
        move_element(values, i)?;
        // let vals: Vec<i16> = values.iter().map(|e| e.value).collect();
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
    if moved_element.value() < 0 && moved_element.new_position > moved_element.current_position {
        values[moved_element.current_position..moved_element.new_position].rotate_left(1);
    } else if moved_element.new_position < moved_element.current_position {
        values[moved_element.new_position..=moved_element.current_position].rotate_right(1);
    } else {
        values[moved_element.current_position..=moved_element.new_position].rotate_left(1);
    };
    Ok(())
}

fn compute_result(values: &Vec<Element>) -> anyhow::Result<i16> {
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

static INPUT_FILE: &str = "../inputs/day_20_test.input";

fn main() -> anyhow::Result<()> {
    let mut values: Vec<Element> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .enumerate()
        .map(|(i, r)| {
            Ok(Element {
                value: r?,
                initial_position: i,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    mix(&mut values)?;

    println!("After mixing: {values:?}");

    let result = compute_result(&values);

    println!("The result is {result:?}");

    Ok(())
}
