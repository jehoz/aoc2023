use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let part1 = parse_records(input)
        .iter()
        .map(ways_to_beat)
        .fold(1, |a, b| a * b)
        .to_string();

    let part2 = ways_to_beat(&parse_single_record(input)).to_string();

    Solution { part1, part2 }
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let det = f64::sqrt(b.powi(2) - 4.0 * a * c);
    (((-b) + det) / (2.0 * a), ((-b) - det) / (2.0 * a))
}

fn ways_to_beat(r: &RaceRecord) -> u64 {
    let (s1, s2) = quadratic_roots(-1.0, r.time as f64, -(r.distance as f64));
    (f64::ceil(s2) - f64::floor(s1) - 1.0) as u64
}

struct RaceRecord {
    time: u64,
    distance: u64,
}

fn parse_nums<'a>(prefix: &str, input: &'a str) -> IResult<&'a str, Vec<u64>> {
    preceded(
        pair(tag(prefix), space1),
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>())),
    )(input)
}

fn parse_records(input: &str) -> Vec<RaceRecord> {
    let mut lines = input.lines();
    let (_, times) = parse_nums("Time:", lines.next().unwrap()).unwrap();
    let (_, distances) = parse_nums("Distance:", lines.next().unwrap()).unwrap();

    times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| RaceRecord {
            time: t,
            distance: d,
        })
        .collect()
}

fn parse_single_record(input: &str) -> RaceRecord {
    let parse = |line: &str| {
        line.chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    };

    let mut lines = input.lines();
    let time = parse(lines.next().unwrap());
    let distance = parse(lines.next().unwrap());

    RaceRecord { time, distance }
}
