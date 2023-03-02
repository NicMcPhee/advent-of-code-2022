#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{
    fmt::Display,
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

const NUM_SCREEN_ROWS: usize = 6;
const NUM_SCREEN_COLS: usize = 40;

#[derive(Debug)]
struct Screen {
    rows: [[char; NUM_SCREEN_COLS]; NUM_SCREEN_ROWS],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            rows: [[' '; NUM_SCREEN_COLS]; NUM_SCREEN_ROWS],
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
struct Cpu {
    x: isize,
    program: Vec<Instruction>,
    current_instruction: Instruction,
    current_clock_cycle: usize,
    current_instruction_remaining_cycles: usize,
    screen: Screen,
}

impl Cpu {
    fn new(mut program: Vec<Instruction>) -> Self {
        program.reverse();
        let mut cpu = Self {
            x: 1,
            program,
            current_clock_cycle: 1,
            ..Default::default()
        };
        cpu.load_instruction();
        cpu
    }

    fn load_instruction(&mut self) {
        // The `unwrap_or` says the next instruction is a `Noop` just in case we run past
        // the end of the program.
        self.current_instruction = self.program.pop().unwrap_or(Instruction::Noop);
        self.current_instruction_remaining_cycles = self.current_instruction.cycles();
    }

    // This is a cheap hack, but I really don't want to "properly" deal with the cast at the moment.
    #[allow(clippy::cast_possible_wrap)]
    fn draw(&mut self, row: usize, col: usize) {
        self.screen.rows[row][col] = if (self.x - 1..=self.x + 1).contains(&(col as isize)) {
            '#'
        } else {
            '.'
        };
    }

    fn tick(&mut self, row: usize, col: usize) {
        self.current_clock_cycle += 1;
        self.current_instruction_remaining_cycles -= 1;
        self.draw(row, col);
        if self.current_instruction_remaining_cycles == 0 {
            match self.current_instruction {
                Instruction::Noop => {}
                Instruction::AddX(val_to_add) => self.x += val_to_add,
            }
            self.load_instruction();
        }
    }
}

fn main() -> Result<()> {
    let instructions = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut cpu: Cpu = Cpu::new(instructions);
    for row in 0..NUM_SCREEN_ROWS {
        for col in 0..NUM_SCREEN_COLS {
            cpu.tick(row, col);
        }
    }
    println!("{}", cpu.screen);

    Ok(())
}
