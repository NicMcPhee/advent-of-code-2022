#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(dead_code)]

use std::{collections::HashMap, fs};

use anyhow::Context;

#[derive(Clone)]
enum Monkey {
    Value(i32),
    Expression(Operation, String, String),
}

#[derive(Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// TODO: Possibly add a second `HashMap<String, i32>` that collects
//   the known values of given monkeys. Then `get_value` looks there first,
//    returning the value stored there if there is one. Otherwise it does the
//    math and puts the result in that map before returning.
struct Monkeys {
    monkeys: HashMap<String, Monkey>,
}

impl Monkeys {
    fn get_value(&mut self, monkey_name: &String) -> anyhow::Result<i32> {
        let monkey = self
            .monkeys
            .get(monkey_name)
            .with_context(|| {
                format!("Couldn't find monkey with name {monkey_name} in the hash map.")
            })?
            .clone();
        match monkey {
            Monkey::Value(value) => Ok(value),
            Monkey::Expression(operation, x_name, y_name) => {
                let x = self.get_value(&x_name)?;
                let y = self.get_value(&y_name)?;
                let result = match operation {
                    Operation::Add => x + y,
                    Operation::Subtract => x - y,
                    Operation::Multiply => x * y,
                    Operation::Divide => x / y,
                };
                let entry = self.monkeys.get_mut(monkey_name).with_context(|| {
                    format!("Couldn't find monkey with name {monkey_name} in the hash map")
                })?;
                *entry = Monkey::Value(result);
                Ok(result)
            }
        }
    }
}

static INPUT_FILE: &str = "../inputs/day_21_test.input";

fn main() -> anyhow::Result<()> {
    // let monkeys: Monkeys = fs::read_to_string(INPUT_FILE)
    //     .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
    //     .trim()
    //     .lines()
    //     .map(str::parse);

    // let mut values: Vec<Element> = fs::read_to_string(INPUT_FILE)
    //     .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
    //     .trim()
    //     .lines()
    //     .map(str::parse::<i64>)
    //     .enumerate()
    //     .map(|(i, r)| {
    //         Ok(Element {
    //             value: r? * 811_589_153,
    //             initial_position: i,
    //         })
    //     })
    //     .collect::<anyhow::Result<Vec<Element>>>()?;

    // for _ in 0..10 {
    //     mix(&mut values)?;
    // }

    // println!("After mixing: {values:?}");

    // let result = compute_result(&values);

    // println!("The result is {result:?}");

    Ok(())
}
