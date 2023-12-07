use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::{map_opt, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (_, mappings) =
        separated_list1(tuple((newline, newline)), parse_mapping)(input.trim()).unwrap();

    let part1 = seeds
        .iter()
        .map(|seed| {
            let mut n = seed.clone();
            for mapping in &mappings {
                n = mapping.convert(n);
            }
            n
        })
        .reduce(u32::min)
        .unwrap()
        .to_string();
    let part2 = String::from("Not implemented.");

    Solution { part1, part2 }
}

struct Range {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

struct Mapping {
    ranges: Vec<Range>,
}

impl Mapping {
    fn convert(&self, src: u32) -> u32 {
        for range in &self.ranges {
            if src >= range.source_start && src <= range.source_start + range.length {
                return (src - range.source_start) + range.destination_start;
            }
        }
        src
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tag("seeds: "),
        separated_list1(space1, map_res(digit1, |n: &str| n.parse::<u32>())),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    map_opt(
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>())),
        |ns| {
            Some(Range {
                destination_start: ns.get(0).cloned()?,
                source_start: ns.get(1).cloned()?,
                length: ns.get(2).cloned()?,
            })
        },
    )(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(input)?;
    map_opt(separated_list1(newline, parse_range), |ranges| {
        Some(Mapping { ranges })
    })(input)
}
