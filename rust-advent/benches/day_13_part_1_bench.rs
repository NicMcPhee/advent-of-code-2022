use criterion::{criterion_group, criterion_main, Criterion};

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

use anyhow::Context;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}
#[derive(Debug, Clone)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

// Original version
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.partial_cmp(r),
            (Self::List(ls), Self::List(rs)) => ls.partial_cmp(rs),
            (Self::Value(l), Self::List(rs)) => vec![Self::Value(*l)].partial_cmp(rs),
            (Self::List(ls), Self::Value(r)) => ls.partial_cmp(&vec![Self::Value(*r)]),
        }
    }
}

// // ikopor@Twitch's version
// impl PartialOrd for Packet {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (Self::Value(l), Self::Value(r)) => l.partial_cmp(r),
//             (Self::List(ls), Self::List(rs)) => ls.partial_cmp(rs),
//             (Self::Value(l), Self::List(r)) => {
//                 let l: &[Packet] = &[Self::Value(*l)];
//                 l.partial_cmp(r)
//             }
//             (Self::List(_), Self::Value(_)) => other.partial_cmp(self).map(Ordering::reverse),
//         }
//     }
// }

// // esitsu@Twitch's version
// impl PartialOrd for Packet {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match (self, other) {
//             (Self::Value(l), Self::Value(r)) => l.partial_cmp(r),
//             (Self::List(l), Self::List(r)) => l.partial_cmp(r),
//             (Self::Value(l), r @ Self::List(_)) => l.partial_cmp(r),
//             (l @ Self::List(_), Self::Value(r)) => l.partial_cmp(r),
//         }
//     }
// }

// impl PartialEq<Packet> for u8 {
//     fn eq(&self, other: &Packet) -> bool {
//         match other {
//             Packet::Value(val) => self == val,
//             Packet::List(_) => false,
//         }
//     }
// }

// impl PartialOrd<Packet> for u8 {
//     fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
//         match other {
//             Packet::Value(val) => self.partial_cmp(val),
//             Packet::List(list) => match &list[..] {
//                 [] => Some(Ordering::Greater),
//                 [item] => self.partial_cmp(item),
//                 [item, ..] => match self.partial_cmp(item) {
//                     Some(Ordering::Equal) => Some(Ordering::Less),
//                     ord => ord,
//                 }
//             }
//         }
//     }
// }

// impl PartialEq<u8> for Packet {
//     fn eq(&self, other: &u8) -> bool {
//         other == self
//     }
// }

// impl PartialOrd<u8> for Packet {
//     fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
//         other.partial_cmp(self).map(Ordering::reverse)
//     }
// }

impl PacketPair {
    fn new((left, right): (Packet, Packet)) -> Self {
        Self { left, right }
    }

    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

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

fn compute_sum(packet_pairs: Vec<PacketPair>) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, packet_pair)| packet_pair.is_ordered()) // Give us just the correctly ordered pairs
        .map(|(i, _)| i + 1) // Give us the indices+1 of correctly ordered pairs
        .sum::<usize>()
}

static INPUT_FILE: &str = "../inputs/day_13.input";

fn compute_sum_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))
        .unwrap();

    let (_, packet_pairs) = packet_pair_list(&contents)
        .map_err(|e| e.to_owned())
        .unwrap();

    let result = compute_sum(packet_pairs);

    println!("The final sum was {result}.");

    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))
        .unwrap();

    let (_, packet_pairs) = packet_pair_list(&contents)
        .map_err(|e| e.to_owned())
        .unwrap();

    c.bench_function("compute_sum", |b| {
        b.iter(|| compute_sum(packet_pairs.clone()))
    });
}

criterion_group!(day_13_part_1_bench, compute_sum_benchmark);
criterion_main!(day_13_part_1_bench);
