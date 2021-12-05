use aoc_runner_derive::*;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .filter_map(|pair_str| {
                    pair_str
                        .split(",")
                        .filter_map(|num| num.parse().ok())
                        .next_tuple()
                })
                .next_tuple()
        })
        .collect()
}

#[aoc(day5, part1)]
fn vent_overlaps_horiz_vert(input: &Vec<((usize, usize), (usize, usize))>) -> usize {
    vent_overlaps(input, true)
}

#[aoc(day5, part2)]
fn vent_overlaps_all(input: &Vec<((usize, usize), (usize, usize))>) -> usize {
    vent_overlaps(input, false)
}

fn vent_overlaps(input: &Vec<((usize, usize), (usize, usize))>, only_hor_vert: bool) -> usize {
    let mut grid: HashMap<(usize, usize), u8> = HashMap::new();
    let mut sum = 0;
    input
        .iter()
        .filter(|(a, b)| !only_hor_vert || (a.0 == b.0 || a.1 == b.1))
        .for_each(|(a, b)| {
            let x_inc: i32 = if a.0 == b.0 {
                0
            } else if a.0 > b.0 {
                -1
            } else {
                1
            };
            let y_inc: i32 = if a.1 == b.1 {
                0
            } else if a.1 > b.1 {
                -1
            } else {
                1
            };
            let len = max(
                (a.0 as i32 - b.0 as i32).abs(),
                (a.1 as i32 - b.1 as i32).abs(),
            );
            let mut i = 0i32;
            while i <= len {
                let idx_x = (a.0 as i32 + i * x_inc) as usize;
                let idx_y = (a.1 as i32 + i * y_inc) as usize;
                let val = grid.get(&(idx_x, idx_y)).unwrap_or(&0) + 1;
                if val == 2 {
                    sum += 1;
                }
                grid.insert((idx_x, idx_y), val);
                i += 1;
            }
        });

    sum
}
