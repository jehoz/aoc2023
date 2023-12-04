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

pub fn part2(input: &String) -> String {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut modified_input = String::new();

    for (i, char) in input.chars().enumerate() {
        for (word_i, word) in words.iter().enumerate() {
            if input.get(i..).unwrap().starts_with(word) {
                let digit = format!("{}", word_i + 1);
                modified_input.push_str(digit.as_str());
            }
        }
        modified_input.push(char);
    }

    part1(&modified_input)
}
