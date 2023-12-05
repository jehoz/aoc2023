use curl::easy::Easy;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

mod day01;
mod day02;

struct Solution {
    part1: String,
    part2: String,
}

fn get_input(day: u32) -> Option<String> {
    let path = format!("inputs/{day:02}.txt");

    // try to download input from AoC website if we don't have it on disk
    if !Path::new(&path).exists() {
        println!("Downloading puzzle input...");
        match env::var("AOC_SESSION") {
            Ok(session_id) => {
                let mut req = Easy::new();

                let url = format!("https://adventofcode.com/2023/day/{day}/input");
                req.url(url.as_str()).unwrap();

                let session_cookie = format!("session={session_id}");
                req.cookie(session_cookie.as_str()).unwrap();

                File::create(&path).unwrap();
                let mut file = File::options()
                    .write(true)
                    .append(true)
                    .open(&path)
                    .unwrap();
                req.write_function(move |data| {
                    if let Err(e) = file.write_all(data) {
                        print!("ERROR: {e}");
                    }
                    Ok(data.len())
                })
                .unwrap();

                req.perform().unwrap();
            }

            Err(_) => {
                println!("ERROR: Missing environment variable AOC_SESSION");
                println!("Please add it to the environment or manually download the puzzle input.");
                return None;
            }
        }
    }

    fs::read_to_string(path).ok()
}

pub fn run_solver(day: u32) {
    let solve = match day {
        1 => day01::solve,
        2 => day02::solve,
        other => {
            println!("No solver implemented for day {other}.");
            return;
        }
    };

    let Some(input) = get_input(day) else { return };
    println!("DAY {day}");

    let solution = solve(&input);
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}
