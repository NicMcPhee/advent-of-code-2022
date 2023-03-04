#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs,
    str::{FromStr, Lines},
};

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_11.input";

#[derive(Debug)]
enum Value {
    Old,
    Int(usize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "old" => Self::Old,
            _ => Self::Int(s.parse::<usize>()?),
        })
    }
}

#[derive(Debug)]
struct Expression {
    operator: char,
    right: Value,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Expression,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

fn next_line<'a>(lines: &mut Lines<'a>) -> Result<&'a str> {
    lines
        .next()
        .context("Couldn't get enough lines for a monkey")
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        // We skip the first line because it doesn't have any info we care about.
        next_line(&mut lines)?;
        // TODO: When `next_line` took `&'a mut Lines` we had errors about multiple borrows of
        //   mutable references. When we changed it to `&mut Lines<'a>` (at esitsu@Twitch's suggestion)
        //   the problem went away. I'd like to understand that more fully – do homework.
        Ok(Self {
            items: Self::parse_items(next_line(&mut lines)?)?,
            operation: Self::parse_operation(next_line(&mut lines)?),
            test_value: Self::parse_test(next_line(&mut lines)?),
            true_target: Self::parse_test_branch(next_line(&mut lines)?),
            false_target: Self::parse_test_branch(next_line(&mut lines)?),
        })
    }
}

/*
Monkey 0:
  Starting items: 71, 86
  Operation: new = old * 13
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 7
 */
impl Monkey {
    fn parse_items(s: &str) -> Result<Vec<usize>> {
        // Skip the first 18 characters; this should put us at the start of the
        // list of item numbers.
        Ok(s[18..]
            .split(", ")
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn parse_operation(s: &str) -> Expression {
        // We can skip the first 19 characters, then split into three parts, and ignore
        // the first part because it's always "old" (with this input).
        let mut parts = s[19..].split_ascii_whitespace().skip(1);
        #[allow(clippy::unwrap_used)]
        Expression {
            operator: parts.next().unwrap().chars().next().unwrap(),
            right: parts.next().unwrap().parse::<Value>().unwrap(),
        }
    }

    fn parse_test(s: &str) -> usize {
        #[allow(clippy::unwrap_used)]
        s[21..].parse::<usize>().unwrap()
    }

    fn parse_test_branch(s: &str) -> usize {
        #[allow(clippy::unwrap_used)]
        s.split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }
}

struct MonkeyState {
    monkeys: Vec<Monkey>,
    items: Vec<Vec<usize>>,
    inspection_count: Vec<usize>,
}

impl MonkeyState {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let num_monkeys = monkeys.len();
        Self {
            monkeys,
            items: vec![vec![]; num_monkeys],
            inspection_count: vec![0; num_monkeys],
        }
    }

    fn process_monkeys(self) -> Self {
        let num_monkeys = self.monkeys.len();

        let repeated_monkeys = (0..num_monkeys).cycle().take(20 * num_monkeys);

        repeated_monkeys.fold(self, Self::process_monkey)
    }

    fn process_monkey(self, monkey_number: usize) -> Self {
        todo!()
    }

    fn monkey_business(&self) -> usize {
        todo!()
    }
}

fn main() -> Result<()> {
    let monkeys = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?;

    let state = MonkeyState::new(monkeys);

    let final_state = state.process_monkeys();

    println!("The monkey business was {}", final_state.monkey_business());

    Ok(())
}
