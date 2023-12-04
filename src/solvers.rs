use std::fs;

mod day01;

pub struct Solver {
    input_path: &'static str,
    part1: fn(&String) -> String,
    part2: fn(&String) -> String,
}

pub fn run_solver(day: u32) {
    let solver = match day {
        1 => &day01::SOLVER,
        other => {
            println!("No solver implemented for day {other}.");
            return;
        },
    };

    let input = fs::read_to_string(&solver.input_path).expect("Failed to read input file.");

    println!("DAY {day}");

    let p1_sln = (solver.part1)(&input);
    println!("Part 1: {p1_sln}");

    let p2_sln = (solver.part2)(&input);
    println!("Part 2: {p2_sln}");
}
