use aoc_runner_derive::*;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    let pos = &mut input.clone();
    pos.sort();
    let median = pos[pos.len() / 2];
    pos.iter().map(|p| (*p - median).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<i32>) -> i32 {
    let mean = mean(input) as i32;
    input
        .iter()
        .map(|p| {
            let diff = (*p - mean).abs();
            (1..diff + 1).fold(0, |a, b| a + b)
        })
        .sum()
}

fn mean(list: &Vec<i32>) -> i32 {
    let sum: i32 = list.iter().sum();
    (f64::from(sum) / (list.len() as f64)) as i32
}
