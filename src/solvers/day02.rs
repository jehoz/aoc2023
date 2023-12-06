use std::cmp;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    multi::fold_many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect();

    let p1_ans = games
        .iter()
        .filter(|g| {
            g.observations
                .iter()
                .all(|o| o.red <= 12 && o.green <= 13 && o.blue <= 14)
        })
        .map(|g| g.id)
        .sum::<u32>()
        .to_string();

    let p2_ans = games
        .iter()
        .map(|g| {
            let mut maxes = (0, 0, 0);
            for obs in &g.observations {
                maxes.0 = cmp::max(maxes.0, obs.red);
                maxes.1 = cmp::max(maxes.1, obs.green);
                maxes.2 = cmp::max(maxes.2, obs.blue);
            }
            maxes.0 * maxes.1 * maxes.2
        })
        .sum::<u32>()
        .to_string();

    Solution {
        part1: p1_ans,
        part2: p2_ans,
    }
}

struct Observation {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    observations: Vec<Observation>,
}

fn parse_color_count(input: &str) -> IResult<&str, (&str, u32)> {
    terminated(
        alt((
            tuple((digit1, tag(" red"))),
            tuple((digit1, tag(" green"))),
            tuple((digit1, tag(" blue"))),
        )),
        opt(tag(", ")),
    )(input)
    .map(|(input, (val, color))| (input, (color.trim(), val.parse::<u32>().unwrap())))
}

fn parse_observation(input: &str) -> IResult<&str, Observation> {
    let new_observation = || Observation {
        red: 0,
        green: 0,
        blue: 0,
    };

    fold_many1(
        preceded(opt(tag(", ")), parse_color_count),
        new_observation,
        |mut obs, (color, num)| {
            match color {
                "red" => obs.red = num,
                "green" => obs.green = num,
                "blue" => obs.blue = num,
                _ => panic!(),
            }
            obs
        },
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, game_id) = delimited(tag("Game "), digit1, tag(": "))(input)?;

    let new_game = || Game {
        id: game_id.parse::<u32>().unwrap(),
        observations: Vec::new(),
    };

    fold_many1(
        preceded(opt(tag("; ")), parse_observation),
        new_game,
        |mut game, obs| {
            game.observations.push(obs);
            game
        },
    )(input)
}
