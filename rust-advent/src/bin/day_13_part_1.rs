#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{sequence::delimited, IResult};
use std::cmp::Ordering;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}
#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

// ikopor@Twitch's version
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.partial_cmp(r),
            (Self::List(ls), Self::List(rs)) => ls.partial_cmp(rs),
            (Self::Value(l), Self::List(r)) => {
                let l: &[Self] = &[Self::Value(*l)];
                l.partial_cmp(r)
            }
            (Self::List(_), Self::Value(_)) => other.partial_cmp(self).map(Ordering::reverse),
        }
    }
}

impl PacketPair {
    fn new((left, right): (Packet, Packet)) -> Self {
        Self { left, right }
    }

    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

/*
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
 */

fn packet_pair_list(s: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list0(tag("\n\n"), packet_pair)(s)
}

fn packet_pair(s: &str) -> IResult<&str, PacketPair> {
    map(separated_pair(packet, newline, packet), PacketPair::new)(s)
}

fn element_list(s: &str) -> IResult<&str, Vec<Packet>> {
    separated_list0(char(','), alt((map(u8, Packet::Value), packet)))(s)
}

fn packet(s: &str) -> IResult<&str, Packet> {
    map(delimited(char('['), element_list, char(']')), Packet::List)(s)
}

fn compute_sum(packet_pairs: &[PacketPair]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, packet_pair)| packet_pair.is_ordered()) // Give us just the correctly ordered pairs
        .map(|(i, _)| i + 1) // Give us the indices+1 of correctly ordered pairs
        .sum::<usize>()
}

static INPUT_FILE: &str = "../inputs/day_13.input";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (_, packet_pairs) = packet_pair_list(&contents).map_err(|e| e.to_owned())?;

    let result = compute_sum(&packet_pairs);

    println!("The final sum was {result}.");

    Ok(())
}
