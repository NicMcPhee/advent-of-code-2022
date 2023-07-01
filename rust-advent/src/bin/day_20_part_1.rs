#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;

// Have a Vector of (value, initial_position).

#[derive(Debug)]
struct Element {
    value: i16,
    initial_position: usize,
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

fn mix(values: &mut Vec<Element>) -> anyhow::Result<()> {
    let length = values.len();
    let length_i16 =
        i16::try_from(length).with_context(|| format!("Converting {length} to i16 failed."))?;
    for i in 0..length {
        move_element(values, i, length_i16)?;

        // match new_position_usize {
        //     Ok(_) if value == 0 => {}
        //     Ok(new_pos) if value > 0 && new_pos < length => {
        //         values[element_position..=new_pos].rotate_left(1);
        //     }
        //     Err(_) if value < 0 => {
        //         values[element_position
        //             ..usize::try_from(new_position.rem_euclid(length_i16)).with_context(|| {
        //                 format!(
        //                     "Converting new_position {} to usize failed.",
        //                     new_position.rem_euclid(length_i16)
        //                 )
        //             })?]
        //             .rotate_right(1);
        //     }
        //     Ok(new_pos) if value >= 0 && new_pos >= length => {
        //         values[new_pos.rem_euclid(length)..element_position].rotate_right(1);
        //     }
        //     Ok(new_pos) if value < 0 => {
        //         values[new_pos..element_position].rotate_left(1);
        //     }
        //     _ => bail!("Element {element:?} in position {element_position} led to a failed match"),
        // }
        // let vals: Vec<i16> = values.iter().map(|e| e.value).collect();
        // println!("Current vals = {vals:?}");
    }
    Ok(())
}

fn move_element(values: &mut [Element], i: usize, length_i16: i16) -> anyhow::Result<()> {
    let element_position = values
        .iter()
        .position(|e| e.initial_position == i)
        .with_context(|| format!("Failed to find element with initial position {i}."))?;
    let element = values
        .get(element_position)
        .with_context(|| format!("Retrieving element at index {element_position} failed"))?;
    let value = element.value;
    let element_position_i16 = i16::try_from(element_position)
        .with_context(|| format!("Converting {element_position} to i16 failed."))?;
    let mut new_position = (element_position_i16 + value).rem_euclid(length_i16);
    if new_position == 0 && value < 0 {
        new_position += length_i16 - 1;
    }
    let new_position_usize = usize::try_from(new_position)
        .with_context(|| format!("Converting new_position {new_position} to usize failed."))?;
    if value < 0 && new_position > element_position_i16 {
        values[element_position..new_position_usize].rotate_left(1);
    } else if new_position < element_position_i16 {
        values[new_position_usize..=element_position].rotate_right(1);
    } else {
        values[element_position..=new_position_usize].rotate_left(1);
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
