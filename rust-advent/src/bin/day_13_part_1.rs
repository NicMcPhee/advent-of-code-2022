use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{sequence::delimited, IResult};
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}
#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn new((left, right): (Packet, Packet)) -> Self {
        Self { left, right }
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

static INPUT_FILE: &str = "../inputs/day_13_test.input";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    // If we use `?` instead of `unwrap()` then the `Err` variant can contain
    // pointers into `contents` which can create lifetime issues.
    let (_, packet_pairs) = packet_pair_list(&contents).map_err(|e| e.to_owned())?;

    println!("{packet_pairs:#?}");

    Ok(())
}
