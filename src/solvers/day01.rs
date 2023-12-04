use crate::solvers::Solver;

pub static SOLVER: Solver = Solver {
    input_path: "inputs/01.txt",
    part1,
    part2,
};

pub fn part1(input: &String) -> String {
    let line_nums: Vec<String> = input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect())
        .collect();

    line_nums
        .iter()
        .map(|ln| ln.chars().nth(0).unwrap())
        .zip(line_nums.iter().map(|ln| ln.chars().last().unwrap()))
        .map(|(x, y)| format!("{x}{y}").parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

pub fn part2(_input: &String) -> String {
    "Not implemented".to_string()
}
