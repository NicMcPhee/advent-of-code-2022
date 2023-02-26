#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fs::{self},
    str::FromStr,
};

use anyhow::{Context, Result};

static INPUT_FILE: &str = "../inputs/day_10.input";

enum Instruction {
    Noop,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    // noop
    // addx 5
    fn from_str(line: &str) -> Result<Self> {
        if line == "noop" {
            return Ok(Self::Noop);
        }
        let value = line
            .split_ascii_whitespace()
            .nth(1)
            .with_context(|| "No value field in line '{line}'.")?
            .parse::<isize>()?;
        Ok(Self::AddX(value))
    }
}

// (Completed, addx v) -> InProgress(v) -> (Completed, next instruction)
enum CpuState {
    InProgress(isize),
    Completed,
}

struct Cpu {
    x: isize,
    program: Vec<Instruction>,
    step_number: usize,
    state: CpuState,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            x: 1,
            program,
            step_number: 0,
            state: CpuState::Completed,
        }
    }

    fn advance_to(&mut self, step_number: usize) {
        for i in self.step_number..step_number {}
        todo!()
    }
}

fn main() -> Result<()> {
    let instructions = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut cpu: Cpu = Cpu::new(instructions);
    let mut total_signal_strength = 0;
    for target in (20..=220).step_by(40) {
        cpu.advance_to(target);
        total_signal_strength += isize::try_from(cpu.step_number)? * cpu.x;
    }

    println!("The total signal strength was {total_signal_strength}");

    Ok(())
}
