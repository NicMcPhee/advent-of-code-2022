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

#[derive(Default, Debug)]
enum Instruction {
    #[default]
    Noop,
    AddX(isize),
}

impl Instruction {
    const fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

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

#[derive(Default, Debug)]
struct Cpu {
    x: isize,
    program: Vec<Instruction>,
    current_clock_cycle: usize,
    current_instruction: Instruction,
    current_instruction_remaining_cycles: usize,
}

impl Cpu {
    fn new(mut program: Vec<Instruction>) -> Result<Self> {
        program.reverse();
        let mut cpu = Self {
            x: 1,
            program,
            current_clock_cycle: 1,
            ..Default::default()
        };
        cpu.load_instruction()?;
        Ok(cpu)
    }

    fn load_instruction(&mut self) -> Result<()> {
        self.current_instruction = self.program.pop().context("There were no instructions")?;
        self.current_instruction_remaining_cycles = self.current_instruction.cycles();
        Ok(())
    }

    fn tick(&mut self) -> Result<()> {
        self.current_clock_cycle += 1;
        self.current_instruction_remaining_cycles -= 1;
        if self.current_instruction_remaining_cycles == 0 {
            match self.current_instruction {
                Instruction::Noop => {}
                Instruction::AddX(val_to_add) => self.x += val_to_add,
            }
            self.load_instruction()?;
        }
        Ok(())
    }

    fn advance_to(&mut self, target_clock_cycle: usize) -> Result<()> {
        while self.current_clock_cycle < target_clock_cycle {
            // println!("{self:?}");
            self.tick()?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let instructions = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut cpu: Cpu = Cpu::new(instructions)?;
    let mut total_signal_strength = 0;
    // cpu.advance_to(5)?;
    // println!("{cpu:?}");
    for target in (20..=220).step_by(40) {
        cpu.advance_to(target)?;
        println!("{cpu:?}");
        total_signal_strength += isize::try_from(cpu.current_clock_cycle)? * cpu.x;
    }

    println!("The total signal strength was {total_signal_strength}");

    Ok(())
}
