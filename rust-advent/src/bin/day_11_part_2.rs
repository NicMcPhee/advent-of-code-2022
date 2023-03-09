#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs, mem,
    str::{FromStr, Lines},
};

use anyhow::{Context, Result, bail};

static INPUT_FILE: &str = "../inputs/day_11.input";

#[derive(Debug)]
enum Value {
    Old,
    Int(u64),
}

impl Value {
    const fn evaluate(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Int(value) => *value,
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "old" => Self::Old,
            _ => Self::Int(s.parse::<u64>()?),
        })
    }
}

#[derive(Debug)]
struct Expression {
    // Always '+' or '*'
    operator: char,
    right: Value,
}

/*
 * We can safely mod by the Least Common Multiple (LCM)
 * of all the things we mod by in the tests because that
 * won't change the value of any of those mod calculations.
 * This allows us to keep the size of the worry level under
 * control.
 */
impl Expression {
    fn evaluate(&self, old: u64, lcm: u64) -> Result<u64> {
        let other_value = self.right.evaluate(old);
        match self.operator {
            '+' => Ok((old + other_value) % lcm),
            '*' => Ok((old * other_value) % lcm),
            _ => bail!("Illegal operator character '{}'", self.operator),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Expression,
    test_value: u64,
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
    fn parse_items(s: &str) -> Result<Vec<u64>> {
        // Skip the first 18 characters; this should put us at the start of the
        // list of item numbers.
        Ok(s[18..]
            .split(", ")
            .map(str::parse::<u64>)
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

    fn parse_test(s: &str) -> u64 {
        #[allow(clippy::unwrap_used)]
        s[21..].parse::<u64>().unwrap()
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

#[derive(Debug)]
struct MonkeyState {
    monkeys: Vec<Monkey>,
    inspection_count: Vec<usize>,
    lcm: u64,
}

impl MonkeyState {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self {
            inspection_count: vec![0; monkeys.len()],
            lcm: monkeys.iter().map(|m| m.test_value).product(),
            monkeys,
        }
    }

    fn process_monkeys(self) -> Result<Self> {
        let num_monkeys = self.monkeys.len();

        let mut repeated_monkeys = (0..num_monkeys).cycle().take(10_000 * num_monkeys);

        repeated_monkeys.try_fold(self, Self::process_monkey)
    }

    //  Monkey 0:
    //   Monkey inspects an item with a worry level of 79.
    //     Worry level is multiplied by 19 to 1501.
    //     Monkey gets bored with item. Worry level is divided by 3 to 500.
    //     Current worry level is not divisible by 23.
    //     Item with worry level 500 is thrown to monkey 3.
    //   Monkey inspects an item with a worry level of 98.
    //     Worry level is multiplied by 19 to 1862.
    //     Monkey gets bored with item. Worry level is divided by 3 to 620.
    //     Current worry level is not divisible by 23.
    //     Item with worry level 620 is thrown to monkey 3.

    fn process_monkey(mut self, monkey_number: usize) -> Result<Self> {
        mem::take(&mut self.monkeys[monkey_number].items)
            .into_iter()
            .try_fold(self, |ms, worry_level| {
                ms.process_item(monkey_number, worry_level)
            })
    }

    fn process_item(mut self, monkey_number: usize, worry_level: u64) -> Result<Self> {
        let monkey = &self.monkeys[monkey_number];
        let worry_level = monkey.operation.evaluate(worry_level, self.lcm)?;
        #[allow(clippy::match_bool)]
        let target = match worry_level % monkey.test_value == 0 {
            true => monkey.true_target,
            false => monkey.false_target,
        };
        self.monkeys[target].items.push(worry_level);
        self.inspection_count[monkey_number] += 1;
        Ok(self)
    }

    fn monkey_business(&mut self) -> usize {
        self.inspection_count.sort_unstable();
        self.inspection_count.reverse();
        self.inspection_count[0] * self.inspection_count[1]
    }
}

fn main() -> Result<()> {
    let monkeys = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?;

    let state = MonkeyState::new(monkeys);

    println!("The initial state is {state:?}");

    let mut final_state = state.process_monkeys()?;

    println!("The final state is {final_state:?}");

    println!("The monkey business was {}", final_state.monkey_business());

    Ok(())
}
