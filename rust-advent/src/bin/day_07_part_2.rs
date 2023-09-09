#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

static INPUT_FILE: &str = "../inputs/day_07.input";

// `PathBuf` is the path to a directory.
// `usize` will the the associated size of that directory.
type DirectoryMap = HashMap<PathBuf, usize>;

#[derive(Default)]
struct FileSystemState {
    directory_map: DirectoryMap,
    current_directory: PathBuf,
}

enum InputLine {
    Cd(String),
    Ls,
    // Do we actually need to record the directory name here?
    Dir(String),
    File(usize),
}

impl FromStr for InputLine {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let first_part = parts
            .first()
            .with_context(|| "The line '{line}' didn't have a first part")?;
        let entry = match *first_part {
            "$" => match parts.get(1) {
                Some(&"cd") => Self::Cd(parts[2].to_string()),
                Some(&"ls") => Self::Ls,
                _ => bail!("Unknown command {} (should be 'cd' or 'ls')", parts[1]),
            },
            "dir" => Self::Dir(parts[1].to_string()),
            _ => Self::File(parts[0].parse()?),
        };

        Ok(entry)
    }
}

impl FileSystemState {
    fn process_input_line(self, input_line: InputLine) -> Self {
        // NOTE that we can import all the enum types and just use, e.g, `Dir(directory_name)`
        // instead of `InputLine::Dir(directory_name). Thanks to NathanielBumppo@Twitch for
        // the suggestion. I'm not using it here based in part on a comment from
        // ikopor@Twitch noting that without the `InputLine::` prefix, the `match`
        // could end up thinking we were introducing a variable if there was a typo
        // in something like `Ls`.
        // use InputLine::*;

        match input_line {
            InputLine::Cd(directory_name) => self.handle_cd(directory_name),
            InputLine::Ls | InputLine::Dir(_) => self,
            InputLine::File(size) => self.handle_file(size),
        }
    }

    // TODO: If the directory_name was an enum with three variants (Slash,
    //   DotDot, and Name), then this would be a `match` clause and I wouldn't
    //   have been able to have forgotten about the DotDot case like I did the
    //   first time.
    fn handle_cd(mut self, directory_name: String) -> Self {
        if directory_name == "/" {
            self.current_directory = PathBuf::from("/");
        } else if directory_name == ".." {
            self.current_directory.pop();
        } else {
            self.current_directory.push(directory_name);
        }
        self
    }

    fn handle_file(mut self, file_size: usize) -> Self {
        // If the file size is 10 and the current directory is "/a/b/c"
        // we need to add 10 to "/a/b/c", "/a/b", "/a", and "/".
        for directory in self.current_directory.ancestors() {
            *self
                .directory_map
                .entry(directory.to_path_buf())
                .or_insert(0) += file_size;
        }
        self
    }
}

fn parse_commands(contents: &str) -> Result<FileSystemState> {
    let final_state: FileSystemState = contents
        .lines()
        .map(|line_str| {
            line_str
                .parse::<InputLine>()
                .with_context(|| "The line {line_str} failed to parse to an `InputLine`")
        })
        .try_fold(
            FileSystemState::default(),
            |file_system_state, input_line| -> Result<FileSystemState> {
                Ok(file_system_state.process_input_line(input_line?))
            },
        )?;
    Ok(final_state)
}

fn find_directory_to_delete(file_system_state: &FileSystemState) -> Result<usize> {
    let directory_map = &file_system_state.directory_map;
    let total_used = directory_map
        .get(&PathBuf::from("/"))
        .context("The directory map didn't have an entry for '/'")?;
    let total_free = 70_000_000 - total_used;
    let total_needed = 30_000_000 - total_free;
    let result = file_system_state
        .directory_map
        .iter()
        .filter_map(|(_, size)| {
            if *size >= total_needed {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .copied()
        .context("There were no directories that were big enough")?;
    Ok(result)
}

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let file_system_state: FileSystemState = parse_commands(&contents)?;

    let total_sizes = find_directory_to_delete(&file_system_state)?;

    println!("The total of the sizes was {total_sizes}");

    Ok(())
}
