#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::{sequence::delimited, IResult};
use std::cmp::Ordering;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
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

impl Packet {
    fn divider_packet(val: u8) -> Self {
        Self::List(vec![Self::List(vec![Self::Value(val)])])
    }
}

fn packet_list(s: &str) -> IResult<&str, Vec<Packet>> {
    separated_list0(multispace1, packet)(s)
}

fn element_list(s: &str) -> IResult<&str, Vec<Packet>> {
    separated_list0(char(','), alt((map(u8, Packet::Value), packet)))(s)
}

fn packet(s: &str) -> IResult<&str, Packet> {
    map(delimited(char('['), element_list, char(']')), Packet::List)(s)
}

static INPUT_FILE: &str = "../inputs/day_13.input";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    let (_, packets) = packet_list(&contents).map_err(|e| e.to_owned())?;

    let divider_2 = Packet::divider_packet(2);
    let divider_6 = Packet::divider_packet(6);

    // println!("The packages were {packets:?}");

    let (less_than_2, greater_than_2): (Vec<_>, Vec<_>) =
        packets.into_iter().partition(|p| *p < divider_2);
    let less_than_6 = greater_than_2
        .into_iter()
        .filter(|p| *p < divider_6)
        .count();

    let divider_2_pos = less_than_2.len() + 1;
    let divider_6_pos = divider_2_pos + less_than_6 + 1;

    println!(
        "Pos of 2 was {divider_2_pos}, pos of 6 was {divider_6_pos} and product was {}",
        divider_2_pos * divider_6_pos
    );

    Ok(())
}
