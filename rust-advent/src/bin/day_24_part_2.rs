#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::items_after_test_module)]

use anyhow::Context;
use pathfinding::directed::astar::astar;
use std::fmt::Display;
use std::iter::once;
use std::ops::{Add, Not};
use std::{collections::HashMap, fs};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use vector2d::Vector2D;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    const fn manhattan_dist_to(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(
        self,
        Self {
            row: delta_row,
            col: delta_col,
        }: Self,
    ) -> Self::Output {
        Self {
            row: self.row + delta_row,
            col: self.col + delta_col,
        }
    }
}

impl From<Vector2D<usize>> for Pos {
    fn from(v: Vector2D<usize>) -> Self {
        Self { row: v.x, col: v.y }
    }
}

impl Add<Direction> for Pos {
    type Output = Option<Self>;

    // The position will always be "inside" the walls, which means that
    // that both `row` and `col` are guaranteed to be > 0. This makes
    // subtracting one "safe" here.
    fn add(self, direction: Direction) -> Self::Output {
        Some(match direction {
            Direction::North => Self {
                row: self.row.checked_sub(1)?,
                col: self.col,
            },
            Direction::South => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::West => Self {
                row: self.row,
                col: self.col.checked_sub(1)?,
            },
            Direction::East => Self {
                row: self.row,
                col: self.col + 1,
            },
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    pos: Pos,
    time: usize,
}

impl Node {
    const fn new(pos: Pos, time: usize) -> Self {
        Self { pos, time }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const fn rotate_180(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

impl From<Direction> for Vector2D<isize> {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => Self::new(-1, 0),
            Direction::South => Self::new(1, 0),
            Direction::West => Self::new(0, -1),
            Direction::East => Self::new(0, 1),
        }
    }
}

impl From<&Pos> for Vector2D<usize> {
    fn from(pos: &Pos) -> Self {
        Self::new(pos.row, pos.col)
    }
}

trait RemEuclid {
    type Output;

    fn rem_euclid(&self, bounds: Self) -> Self::Output;
}

impl RemEuclid for Vector2D<isize> {
    type Output = Vector2D<usize>;

    fn rem_euclid(
        &self,
        Self {
            x: x_bounds,
            y: y_bounds,
        }: Self,
    ) -> Self::Output {
        Vector2D::new(
            self.x.rem_euclid(x_bounds) as usize,
            self.y.rem_euclid(y_bounds) as usize,
        )
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "^"),
            Self::South => write!(f, "v"),
            Self::West => write!(f, "<"),
            Self::East => write!(f, ">"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    blizzards: HashMap<Pos, Blizzard>,
    num_rows: usize,
    num_cols: usize,
    start: Pos,
    finish: Pos,
}

impl Map {
    fn blizzard_at(&self, position: &Pos) -> Option<Blizzard> {
        self.blizzards.get(position).copied()
    }

    fn legal_position(&self, position: &Pos) -> bool {
        // We need to be able to move "down" into the wall when the
        // position is the target position.
        if *position == self.start || *position == self.finish {
            return true;
        }
        position.row > 0
            && (position.row < self.num_rows - 1)
            && position.col > 0
            && position.col < self.num_cols - 1
    }

    #[allow(clippy::cast_possible_wrap)]
    fn initial_pos(&self, position: &Pos, dir: Direction, time: usize) -> Pos {
        let direction_vector: Vector2D<isize> = Vector2D::from(dir) * (time as isize);
        Pos::from(
            (Vector2D::from(position).as_isizes() + Vector2D::new(-1, -1) + direction_vector)
                .rem_euclid(Vector2D::new(
                    self.num_rows as isize - 2,
                    self.num_cols as isize - 2,
                ))
                + Vector2D::new(1, 1),
        )
    }

    fn no_blizzard(&self, position: &Pos, time: usize) -> bool {
        if *position == self.finish || *position == self.start {
            return true;
        }
        Direction::iter()
            .any(|dir| {
                let pos = self.initial_pos(position, dir, time);

                self.blizzard_at(&pos)
                    .is_some_and(|blizzard| blizzard.direction.rotate_180() == dir)
            })
            .not()
    }

    // The plan is to use MizardX@Twitch's idea of wrapping, so we leave
    // the map unchanged, and just move the blizzards `t` time steps
    // in their direction and see if they get in the way.
    //
    // We know where we are, so we know which positions problematic
    // blizzards could be in, and we can reverse time to find out where
    // those blizzards would have needed to be in the initial map, and
    // then just look them up.
    fn successors(&self, Node { pos, time }: Node) -> impl IntoIterator<Item = (Node, usize)> + '_ {
        // println!("{pos:?}");
        Direction::iter()
            .filter_map(move |dir| pos + dir)
            .filter(|pos| self.legal_position(pos))
            .chain(once(pos))
            .filter(move |pos| self.no_blizzard(pos, time + 1))
            .map(move |pos| {
                (
                    Node {
                        pos,
                        time: time + 1,
                    },
                    1,
                )
            })
    }

    const fn dist_to_goal(node: &Node, finish: &Pos) -> usize {
        node.pos.manhattan_dist_to(finish)
    }

    fn finished(node: &Node, finish: &Pos) -> bool {
        node.pos == *finish
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn successors_test() {
        let mut map = Map {
            blizzards: HashMap::new(),
            num_rows: 10,
            num_cols: 10,
            start: Pos { row: 0, col: 1 },
            finish: Pos { row: 9, col: 8 },
        };
        map.blizzards.insert(
            Pos { row: 5, col: 5 },
            Blizzard {
                direction: Direction::North,
            },
        );

        let node = Node {
            pos: Pos { row: 3, col: 5 },
            time: 0,
        };

        let successors = map.successors(node).into_iter().collect::<Vec<_>>();
        dbg!(&successors);
        assert_eq!(successors.len(), 4);
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn no_blizzard_test() {
        let mut map = Map {
            blizzards: HashMap::new(),
            num_rows: 10,
            num_cols: 10,
            start: Pos { row: 0, col: 1 },
            finish: Pos { row: 9, col: 8 },
        };
        map.blizzards.insert(
            Pos { row: 5, col: 5 },
            Blizzard {
                direction: Direction::North,
            },
        );

        let pos = Pos { row: 4, col: 5 };
        let above = (pos + Direction::North).unwrap();
        let below = (pos + Direction::South).unwrap();

        assert_eq!(
            [
                map.no_blizzard(&pos, 1),
                map.no_blizzard(&above, 1),
                map.no_blizzard(&below, 1)
            ],
            [false, true, true]
        );
    }
}

#[allow(clippy::expect_used)]
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0usize..self.num_rows {
            for col in 0..self.num_cols {
                let pos = Pos::new(row, col);
                if pos != self.start
                    && pos != self.finish
                    && (row == 0
                        || col == 0
                        || row == self.num_rows - 1
                        || col == self.num_cols - 1)
                {
                    write!(f, "#")?;
                    continue;
                }
                match self.blizzard_at(&pos) {
                    None => write!(f, ".")?,
                    Some(blizzard) => write!(f, "{}", blizzard.direction)?,
                    // [] => write!(f, ".")?,
                    // [b] => write!(f, "{}", b.direction)?,
                    // blizzards => write!(f, "{}", &blizzards.len())?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn process_line(blizzards: &mut HashMap<Pos, Blizzard>, row: usize, line: &str) {
    for (col, c) in line.chars().enumerate() {
        match c {
            '#' | '.' => {}
            _ => {
                let direction = match c {
                    '<' => Direction::West,
                    '>' => Direction::East,
                    '^' => Direction::North,
                    'v' => Direction::South,
                    _ => unreachable!("We received a character {c} that shouldn't have happened"),
                };
                blizzards.insert(Pos::new(row, col), Blizzard { direction });
            }
        }
    }
}

fn parse_map(file_contents: &str) -> Map {
    let mut blizzards: HashMap<Pos, Blizzard> = HashMap::new();
    let mut num_rows = usize::MIN;
    let mut num_cols = 0;
    for (row, line) in file_contents.lines().enumerate() {
        process_line(&mut blizzards, row, line);
        num_rows = row;
        num_cols = line.len();
    }
    Map {
        blizzards,
        num_rows: num_rows + 1,
        num_cols,
        start: Pos::new(0, 1),
        finish: Pos::new(num_rows, num_cols - 2),
    }
}

fn do_search(
    map: &Map,
    start: &Pos,
    finish: &Pos,
    start_time: usize,
) -> Option<(Vec<Node>, usize)> {
    astar(
        &Node::new(*start, start_time),
        |node| map.successors(*node),
        |node| Map::dist_to_goal(node, finish),
        |node| Map::finished(node, finish),
    )
}

static INPUT_FILE: &str = "../inputs/day_24.input";

fn main() -> anyhow::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let map = parse_map(&file);
    println!("Initial map: \n{map}");
    println!("{}, {}", map.num_rows, map.num_cols);
    println!("{:?}, {:?}", map.start, map.finish);

    let Some((_, first_time)) = do_search(&map, &map.start, &map.finish, 0) else {
        unreachable!("Failed to find the first path");
    };
    dbg!(first_time);
    let Some((_, second_time)) = do_search(&map, &map.finish, &map.start, first_time) else {
        unreachable!("Failed to find the second path");
    };
    dbg!(second_time);
    let Some((_, third_time)) = do_search(&map, &map.start, &map.finish, first_time + second_time)
    else {
        unreachable!("Failed to find the third path");
    };
    dbg!(third_time);

    // println!("The path was {path:#?}");

    println!(
        "The number of minutes was {}.",
        first_time + second_time + third_time
    );

    Ok(())
}
