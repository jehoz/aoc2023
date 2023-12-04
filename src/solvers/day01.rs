use crate::solvers::Solver;

pub static SOLVER: Solver = Solver {
    input_path: "inputs/01.txt",
    part1,
    part2,
};

pub fn part1(input: &String) -> String {
    input
        .lines()
        .map(|ln| {
            let nums: String = ln.chars().filter(|c| c.is_digit(10)).collect();
            let first = nums.chars().next().unwrap();
            let last = nums.chars().last().unwrap();
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(_input: &String) -> String {
    "Not implemented".to_string()
}
