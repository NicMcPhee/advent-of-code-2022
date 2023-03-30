use itertools::Itertools;
use std::{fs, str::{FromStr, Bytes}, iter::Peekable};

use anyhow::{Context, Result};

#[derive(Debug)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    // [1,[2,[3,[4,[5,6,7]]]],8,9]
    fn from_str(s: &str) -> Result<Self> {
        fn parse_elements(bs: &mut Peekable<Bytes>) -> Result<Vec<Packet>> {
            let mut packets = Vec::new();
            while bs.peek().is_some() {
                let first_packet = parse_single_element(bs)?;
                packets.push(first_packet);
                if bs.peek() == Some(&b']') {
                    bs.next();
                    break;
                }
                if bs.peek().is_some() {
                    bs.next();
                }
            }
            Ok(packets)
        }

        fn parse_single_element(bs: &mut Peekable<Bytes>) -> Result<Packet> {
            if bs.peek() == Some(&b'[') {
                bs.next();
                let elements = parse_elements(bs)?;
                bs.next(); // Remove the closing ']'
                Ok(Packet::List(elements))
            } else {
                // Maybe use itertools `peeking_take_while`?
                let mut value = 0;
                while let Some(b) = bs.peek() {
                    if b'0' <= *b && *b <= b'9' {
                        value = 10*value + (*b - b'0') as i32;
                        bs.next();
                    } else {
                        break;
                    }
                }
                Ok(Packet::Value(value))
            }
        }

        println!("Parsing {s}");
        if s == "[]" {
            return Ok(Self::List(Vec::new()));
        }
        let first_char = s.bytes().nth(0).context("The string to parse was empty")?;
        if first_char == b'[' {
            let packets = parse_elements(&mut s[1..s.len()].bytes().peekable())?;
            Ok(Self::List(packets))
        } else {
            Ok(Self::Value(s.parse()?))
        }
    }
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl FromStr for PacketPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let left = lines
            .next()
            .context("There were no lines in this packet pair")?
            .parse()?;
        let right = lines
            .next()
            .with_context(|| format!("There was only one line in this packet pair: {s}"))?
            .parse()?;
        Ok(PacketPair { left, right })
    }
}

static INPUT_FILE: &str = "../inputs/day_13_test.input";

fn main() -> Result<()> {
    let packet_pairs: Vec<PacketPair> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .split("\n\n")
        .map(|s| s.parse::<PacketPair>())
        .try_collect()?;

    println!("{packet_pairs:#?}");

    Ok(())
}
