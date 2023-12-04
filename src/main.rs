use std::{fs, println};

fn main() {
    let input = fs::read_to_string("inputs/01.txt").expect("Failed to read input file");

    let line_nums = input.lines().map(|l| {
        l.chars()
            .filter(|c| c.is_digit(10) || c.is_whitespace())
            .map(|c| c.to_digit(10).unwrap())
    });

    let ans: u32 = line_nums.clone()
        .map(|mut ln| ln.nth(0).unwrap())
        .zip(line_nums.map(|ln| ln.last().unwrap()))
        .map(|(x, y)| x * 10 + y)
        .sum();

    println!("{ans}");
}
