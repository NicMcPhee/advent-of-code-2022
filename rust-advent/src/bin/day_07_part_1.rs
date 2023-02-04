#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{collections::HashMap, fs, path::PathBuf, str::FromStr};

use anyhow::{bail, Context, Result};

static INPUT_FILE: &str = "../inputs/day_07.input";

type DirectoryMap = HashMap<PathBuf, usize>;

enum InputLine {
    Cd(String),
    Ls,
    Dir(String),
    File(usize),
}

impl FromStr for InputLine {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let entry = match parts[0] {
            "$" => match parts[1] {
                "cd" => InputLine::Cd(parts[2].to_string()),
                "ls" => InputLine::Ls,
                _ => bail!("Unknown command {} (should be 'cd' or 'ls')", parts[1]),
            },
            "dir" => InputLine::Dir(parts[1].to_string()),
            _ => InputLine::File(parts[0].parse()?),
        };

        Ok(entry)
    }
}

fn parse_commands(contents: &str) -> Result<DirectoryMap> {
    let commands = contents.lines();
    todo!()
}

fn sum_of_sizes(s: &DirectoryMap) -> Result<usize> {
    todo!()
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let file_system_map: DirectoryMap = parse_commands(&contents)?;

    let total_sizes = sum_of_sizes(&file_system_map)?;

    println!("The total of the sizes was {total_sizes}");

    Ok(())
}
