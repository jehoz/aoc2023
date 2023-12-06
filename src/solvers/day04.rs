use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let num_matching: Vec<u32> = input
        .lines()
        .map(parse_card)
        .map(|card| {
            card.given_numbers
                .iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count() as u32
        })
        .collect();

    let part1 = num_matching
        .iter()
        .map(|&n| if n == 0 { 0 } else { 2u32.pow(n as u32 - 1) })
        .sum::<u32>()
        .to_string();

    Solution {
        part1,
        part2: "Not implemented".to_string(),
    }
}

struct Card {
    winning_numbers: Vec<u32>,
    given_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> Card {
    let input = input.split_once(":").unwrap().1;

    let mut numbers = input
        .split("|")
        .map(|nums| nums.split_whitespace().map(|n| n.parse::<u32>().unwrap()));

    Card {
        winning_numbers: numbers.next().unwrap().collect(),
        given_numbers: numbers.next().unwrap().collect(),
    }
}
