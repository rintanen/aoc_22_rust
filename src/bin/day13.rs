use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};

use std::cmp::Ordering::{self, *};

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Eq)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}


fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet),
            tag("]"),
        )
        .map(|packets| Packet::List(packets)),
        nom::character::complete::i32
            .map(|number| Packet::Number(number)),
    ))(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet, newline, parse_packet)
            .map(|(left, right)| Pair { left, right },
    ))(input)
}


impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::List(left), Self::Number(right)) => left == &vec![Packet::Number(*right)],
            (Self::Number(left), Self::List(right)) => &vec![Packet::Number(*left)] == right,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self,other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

fn pairs_in_correct_order(pairs: &[Pair]) -> Vec<usize> {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| {
            match left.cmp(right)
            {
                Less | Equal => Some(i),
                Greater => None,
            }
        })
        .collect::<Vec<usize>>()
}

fn sum_of_indices(indices: &[usize]) -> usize {
    indices
        .iter()
        .map(|i| i + 1)
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../../inputs/day13.in");
    let (_, pairs) = parse_pairs(input).unwrap();

    // pt1
    let in_correct_order = pairs_in_correct_order(&pairs);
    let sum = sum_of_indices(&in_correct_order);
    println!("PT1:\npairs in correct order: {in_correct_order:?}\n \
             sum of indices: {sum}");

}