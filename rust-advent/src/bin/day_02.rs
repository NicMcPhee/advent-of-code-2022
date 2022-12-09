use std::fs;

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper,
    Scissors
}

impl Move {
    fn val(&self) -> i32 {
        *self as i32
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("We got an illegal move string")
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Tie = 1,
    Win = 2
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => panic!("We got an illegal outcome string")
        }
    }    
}

impl Outcome {
    fn val(&self) -> i32 {
        *self as i32
    }

    fn score(&self) -> i32 {
        3 * self.val()
    }
}

#[derive(Debug)]
struct MovePair {
    their_move: Move,
    my_move: Move
}

impl FromIterator<Move> for MovePair {
    fn from_iter<T: IntoIterator<Item = Move>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        Self {
            their_move: i.next().unwrap(),
            my_move: i.next().unwrap()
        }
    }
}

impl From<&str> for MovePair {
    fn from(s: &str) -> Self {
        s.split_ascii_whitespace()
         .map(Move::from)
         .collect()
    }
}

impl MovePair {
    fn outcome(&self) -> Outcome {
        let diff: i32 = self.my_move.val() - self.their_move.val();
        match diff {
            1 | -2 => Outcome::Win,
            0 => Outcome::Tie,
            _ => Outcome::Lose
        }
    }

    fn total_score(self) -> i32 {
        self.my_move.val() + self.outcome().score()
    }
}

struct TargetPair {
    their_move: Move,
    outcome: Outcome,
}

impl<'a> FromIterator<&'a str> for TargetPair {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        Self {
            their_move: i
                .next()
                .unwrap()
                .into(),
            outcome: i
                .next()
                .unwrap()
                .into(),
        }
    }
}

impl From<&str> for TargetPair {
    fn from(s: &str) -> Self {
        s.split_ascii_whitespace().collect()
    }
}

impl TargetPair {
    fn compute_my_move(&self) -> Move {
        match self.their_move.val() + self.outcome.val() - 1 {
            1 | 4 => Move::Rock,
            2     => Move::Paper,
            3 | 0 => Move::Scissors,
            diff => panic!("Illegal diff value {diff} for Move", ) 
        }
    }

    fn total_score(self) -> i32 {
        let my_move = self.compute_my_move();
        let moves = MovePair { their_move: self.their_move, my_move };
        my_move.val() + moves.outcome().score()
    }
}

fn main() {
    let contents = fs::read_to_string("../inputs/day_02.input")
        .expect("Should have been able to read the file");

    // println!("{:?}", contents.split("\n").map(MovePair::from).map(MovePair::total_score).take(10).collect::<Vec<i32>>());
    
    let lines = contents.split('\n');

    let total: i32 = lines.clone()
        .map(MovePair::from)
        .map(MovePair::total_score)
        .sum();

    println!("The total was {total}");

    let total_part_2: i32 = lines
        .map(TargetPair::from)
        .map(TargetPair::total_score)
        .sum();

    println!("The second total was {total_part_2}");
}
