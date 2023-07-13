#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Context;
use std::fs;

static INPUT_FILE: &str = "../inputs/day_22_test.input";

fn main() -> anyhow::Result<()> {
    // let monkeys = fs::read_to_string(INPUT_FILE)
    //     .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
    //     .lines()
    //     .map(get_monkey)
    //     .collect::<anyhow::Result<HashMap<MonkeyName, Monkey>>>()?;
    // let mut monkeys = Monkeys { monkeys };

    // println!("{monkeys:?}");

    // let Monkey::Expression(_, left, right) =
    //     monkeys.monkeys.get(&MonkeyName::new("root")).context("Failed to get the root monkey")?.clone()
    // else {
    //     panic!("The root monkey didn't map to an expression")
    // };

    // let left_value = monkeys.get_value(&left)?;
    // let right_value = monkeys.get_value(&right)?;
    // println!("Left = {left_value:?}");
    // println!("Right = {right_value:?}");

    // let difference = left_value - right_value;

    // println!("Difference = {difference:?}");

    // // difference = a + bx
    // // We need a + bx = 0
    // //   == bx = -a
    // //   == x = -a/b

    // let result = -difference.constant / difference.coefficient;

    // println!("Result = {result:?}");

    // Ok(())

    todo!()
}
