use aoc_runner_derive::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    solve(input)
}

#[aoc(day15, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    solve(&duplicate_grid(input, 5))
}

fn duplicate_grid(input: &Vec<Vec<u32>>, num_times: usize) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; input[0].len() * num_times]; input.len() * num_times];
    for i in 0..num_times {
        for j in 0..num_times {
            let offset_x = input.len() * i;
            let offset_y = input[0].len() * j;

            for y in 0..input.len() {
                for x in 0..input[0].len() {
                    let val = (input[x][y] + (i + j) as u32) % 9;
                    grid[x + offset_x][y + offset_y] = if val == 0 { 9 } else { val };
                }
            }
        }
    }
    grid
}

fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut visited = HashSet::new();
    let mut btree = BinaryHeap::new();
    btree.push(Reverse((0, (0, 0))));
    loop {
        let Reverse((risk, (x, y))) = btree.pop().unwrap();
        if !visited.insert((x, y)) {
            continue;
        }
        if (x, y) == (height - 1, width - 1) {
            return risk;
        }
        if x > 0 {
            btree.push(Reverse((risk + input[x - 1][y], (x - 1, y))));
        }
        if y > 0 {
            btree.push(Reverse((risk + input[x][y - 1], (x, y - 1))));
        }
        if x < height - 1 {
            btree.push(Reverse((risk + input[x + 1][y], (x + 1, y))));
        }
        if y < width - 1 {
            btree.push(Reverse((risk + input[x][y + 1], (x, y + 1))));
        }
    }
}
