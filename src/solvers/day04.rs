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

    let part2 = {
        let mut cards_won = vec![1; num_matching.len()];
        for (i, matching) in num_matching.iter().enumerate().rev() {
            for j in 0..matching.clone() as usize {
                cards_won[i] += cards_won[1 + i + j];
            }
        }
        cards_won.iter().sum::<u32>().to_string()
    };

    Solution { part1, part2 }
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
