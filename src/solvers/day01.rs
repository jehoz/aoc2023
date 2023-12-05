use crate::solvers::Solution;

pub fn solve(input: &String) -> Solution {
    Solution {
        part1: input
            .lines()
            .map(first_and_last_digits)
            .sum::<u32>()
            .to_string(),

        part2: input
            .lines()
            .map(|line| {
                let modified_line = words_to_digits(&line);
                first_and_last_digits(modified_line.as_str())
            })
            .sum::<u32>()
            .to_string(),
    }
}

fn first_and_last_digits(line: &str) -> u32 {
    let nums: String = line.chars().filter(|c| c.is_digit(10)).collect();
    let first = nums.chars().next().unwrap();
    let last = nums.chars().last().unwrap();
    format!("{first}{last}").parse::<u32>().unwrap()
}

fn words_to_digits(line: &str) -> String {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut output = String::new();

    for (i, char) in line.chars().enumerate() {
        for (n, word) in words.iter().enumerate() {
            if line.get(i..).unwrap().starts_with(word) {
                let digit = format!("{}", n + 1);
                output.push_str(digit.as_str());
            }
        }
        output.push(char);
    }

    output
}
