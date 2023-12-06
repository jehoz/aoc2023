use std::collections::HashMap;

use nom::{character::complete::digit1, error::Error};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let parts = parse_parts(input);

    let part1 = {
        let mut nums: Vec<PartNumber> = parts
            .iter()
            .flat_map(|p| p.numbers.iter())
            .cloned()
            .collect();
        nums.sort();
        nums.dedup();
        nums.iter().map(|n| n.value).sum::<u32>().to_string()
    };

    let part2 = parts
        .iter()
        .filter(|p| p.symbol == '*' && p.numbers.len() == 2)
        .map(|p| p.numbers[0].value * p.numbers[1].value)
        .sum::<u32>()
        .to_string();

    Solution { part1, part2 }
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
struct PartNumber {
    uid: u32,
    value: u32,
}

struct Part {
    symbol: char,
    numbers: Vec<PartNumber>,
}

fn parse_parts(input: &str) -> Vec<Part> {
    type Coord = (i32, i32);

    let mut uid: u32 = 0;
    let mut num_locs = HashMap::<Coord, PartNumber>::new();
    let mut part_locs = Vec::<(char, Coord)>::new();

    for (y, line) in (0i32..).zip(input.lines()) {
        let mut x = 0;
        while x < line.len() {
            let ch = line.chars().nth(x).unwrap();

            if ch.is_digit(10) {
                let (_, num_str) =
                    digit1::<&str, Error<&str>>(line.get((x as usize)..).unwrap()).unwrap();
                let value = num_str.parse::<u32>().unwrap();
                // map number to coordinate of each of its digits
                for _ in num_str.chars() {
                    num_locs.insert(((x as i32), y), PartNumber { uid, value });
                    x += 1;
                }
                uid += 1;
                continue;
            } else if ch != '.' {
                part_locs.push((ch, ((x as i32), y)));
            }

            x += 1;
        }
    }

    // get any numbers in cells adjacent to part coordinate
    let mut parts = Vec::new();
    for (symbol, (x, y)) in part_locs {
        let mut part = Part {
            symbol,
            numbers: Vec::new(),
        };
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if let Some(n) = num_locs.get(&(x + i, y + j)) {
                    if let None = part.numbers.iter().find(|x| x.uid == n.uid) {
                        part.numbers.push(n.clone());
                    }
                }
            }
        }
        parts.push(part);
    }

    parts
}
