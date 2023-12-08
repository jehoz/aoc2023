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

    let seed_to_loc = chain_mappings(mappings);

    let part1 = seeds
        .iter()
        .map(&seed_to_loc)
        .reduce(u32::min)
        .unwrap()
        .to_string();

    let mut part2 = u32::MAX;
    for range in expand_seed_ranges(seeds) {
        for seed in range {
            let loc = &seed_to_loc(&seed);
            part2 = part2.min(*loc);
        }
    }

    Solution {
        part1,
        part2: part2.to_string(),
    }
}

struct RangeMap {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

struct Mapping {
    ranges: Vec<RangeMap>,
}

impl Mapping {
    fn apply(&self, src: u32) -> u32 {
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

fn parse_range(input: &str) -> IResult<&str, RangeMap> {
    map_opt(
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>())),
        |ns| {
            Some(RangeMap {
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

fn chain_mappings(mappings: Vec<Mapping>) -> impl Fn(&u32) -> u32 {
    move |x: &u32| {
        let mut y = x.clone();
        for mapping in &mappings {
            y = mapping.apply(y);
        }
        y
    }
}

fn expand_seed_ranges(seed_nums: Vec<u32>) -> Vec<std::ops::Range<u32>> {
    let mut ranges = Vec::new();

    for i in (0..seed_nums.len()).step_by(2) {
        let start = seed_nums[i];
        let length = seed_nums[i + 1];
        ranges.push(start..(start + length));
    }

    ranges
}
