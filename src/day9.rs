use aoc_runner_derive::*;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for line_idx in 0..input.len() {
        let line = &input[line_idx];
        for col_idx in 0..line.len() {
            let height = input[line_idx][col_idx];
            if (line_idx == 0 || height < input[line_idx - 1][col_idx])
                && (col_idx == 0 || height < input[line_idx][col_idx - 1])
                && (col_idx + 1 >= line.len() || height < input[line_idx][col_idx + 1])
                && (line_idx + 1 >= input.len() || height < input[line_idx + 1][col_idx])
            {
                sum += height + 1
            }
        }
    }
    sum
}
