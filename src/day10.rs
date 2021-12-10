use aoc_runner_derive::*;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<char>>) -> usize {
    input.iter().map(|line| check_line(&line)).sum()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut x: Vec<usize> = input
        .iter()
        .map(|line| complete_line(&line))
        .filter(|n| n > &0)
        .collect();
    x.sort();
    println!("{:?}", x);
    x[x.len() / 2]
}

fn check_line(line: &Vec<char>) -> usize {
    let mut brackets: Vec<char> = vec![];

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => {
                brackets.push(*c);
            }
            ')' => {
                if brackets.pop() != Some('(') {
                    return 3;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return 57;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return 1197;
                }
            }
            '>' => {
                if brackets.pop() != Some('<') {
                    return 25137;
                }
            }
            _ => {}
        }
    }
    0
}

fn complete_line(line: &Vec<char>) -> usize {
    let mut brackets: Vec<char> = vec![];

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => {
                brackets.push(*c);
            }
            ')' => {
                if brackets.pop() != Some('(') {
                    return 0;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return 0;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return 0;
                }
            }
            '>' => {
                if brackets.pop() != Some('<') {
                    return 0;
                }
            }
            _ => {}
        }
    }
    let mut sum = 0;
    brackets.iter().rev().for_each(|c| {
        sum *= 5;
        match c {
            '(' => sum += 1,
            '[' => sum += 2,
            '{' => sum += 3,
            '<' => sum += 4,
            _ => {}
        }
    });
    sum
}
