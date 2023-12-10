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
    let (_, records) = parse_records(input).unwrap();

    let part1 = records
        .iter()
        .map(|r| {
            let (s1, s2) = quadratic_roots(-1.0, r.time as f32, -(r.distance as f32));
            f32::ceil(s2) - f32::floor(s1) - 1.0
        })
        .fold(1f32, |a, b| a * b)
        .to_string();
    let part2 = "Not implemented".to_string();

    Solution { part1, part2 }
}

fn quadratic_roots(a: f32, b: f32, c: f32) -> (f32, f32) {
    let det = f32::sqrt(b.powi(2) - 4.0 * a * c);
    (((-b) + det) / (2.0 * a), ((-b) - det) / (2.0 * a))
}

struct RaceRecord {
    time: u32,
    distance: u32,
}

fn parse_records(input: &str) -> IResult<&str, Vec<RaceRecord>> {
    let mut lines = input.lines();
    let (_, times) = preceded(
        pair(tag("Time:"), space1),
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>())),
    )(lines.next().unwrap())?;
    let (_, distances) = preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>())),
    )(lines.next().unwrap())?;

    Ok((
        input,
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| RaceRecord {
                time: t,
                distance: d,
            })
            .collect(),
    ))
}
