#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(dead_code)]

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64, one_of, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::{collections::HashMap, fmt::Display, fs, rc::Rc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct MonkeyName(Rc<str>);

impl MonkeyName {
    fn new(s: &str) -> Self {
        Self(Rc::from(s))
    }
}

impl Display for MonkeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Value(i64),
    Expression(Operation, MonkeyName, MonkeyName),
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Monkeys {
    monkeys: HashMap<MonkeyName, Monkey>,
}

fn monkey_name(line: &str) -> IResult<&str, MonkeyName> {
    map(alpha1, MonkeyName::new)(line)
}

fn operator(s: &str) -> IResult<&str, Operation> {
    map(delimited(space1, one_of("+-*/"), space1), |c| match c {
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '*' => Operation::Multiply,
        '/' => Operation::Divide,
        _ => unreachable!("`one_of` given {s} returned an illegal operation character"),
    })(s)
}

fn monkey(s: &str) -> IResult<&str, Monkey> {
    alt((
        map(i64, Monkey::Value),
        map(
            tuple((monkey_name, operator, monkey_name)),
            |(left, op, right)| Monkey::Expression(op, left, right),
        ),
    ))(s)
}

fn parse_monkey(line: &str) -> IResult<&str, (MonkeyName, Monkey)> {
    separated_pair(monkey_name, tag(": "), monkey)(line)
}

fn get_monkey(line: &str) -> anyhow::Result<(MonkeyName, Monkey)> {
    parse_monkey(line)
        // The `<nom::error…>` bit is necessary to specify _which_ `to_owned` we went to use here.
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)
        .with_context(|| format!("Failed to parse line into a monkey: {line}"))
        .map(|(_, m)| m)
}

impl Monkeys {
    fn get_value(&mut self, monkey_name: &MonkeyName) -> anyhow::Result<i64> {
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

static INPUT_FILE: &str = "../inputs/day_21.input";

fn main() -> anyhow::Result<()> {
    let monkeys = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(get_monkey)
        .collect::<anyhow::Result<HashMap<MonkeyName, Monkey>>>()?;
    let mut monkeys = Monkeys { monkeys };

    // println!("{monkeys:?}");

    // Look up and print the value of the monkey named "root".
    let result = monkeys.get_value(&MonkeyName::new("root"))?;

    println!("The result is {result}");

    Ok(())
}
