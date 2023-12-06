use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    sequence::{delimited, tuple},
    IResult,
};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect();

    let part1 = cards
        .iter()
        .map(|card| {
            let matching = card
                .given_numbers
                .iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count();
            if matching == 0 {
                0
            } else {
                2u32.pow(matching as u32 - 1)
            }
        })
        .sum::<u32>()
        .to_string();

    Solution {
        part1,
        part2: "Not implemented".to_string(),
    }
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    given_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card_id) = delimited(tuple((tag("Card"), space1)), digit1, tag(": "))(input)?;

    let mut numbers = input
        .split("|")
        .map(|nums| nums.split_whitespace().map(|n| n.parse::<u32>().unwrap()));

    Ok((
        input,
        Card {
            id: card_id.parse::<u32>().unwrap(),
            winning_numbers: numbers.next().unwrap().collect(),
            given_numbers: numbers.next().unwrap().collect(),
        },
    ))
}
