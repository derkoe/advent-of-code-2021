use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|x| u16::from_str_radix(x, 10).unwrap_or(0))
        .collect()
}

#[aoc(day1, part1)]
fn part1(depths: &Vec<u16>) -> u16 {
    let mut last_depth = u16::max_value();
    let mut increase_count = 0;

    for depth in depths {
        if *depth > last_depth {
            increase_count += 1;
        }
        last_depth = *depth;
    }

    increase_count
}

#[aoc(day1, part2)]
fn part2(depths: &Vec<u16>) -> u16 {
    let mut increase_count = 0;
    let mut last_window = u16::max_value();

    let mut i = 0;
    while i + 2 < depths.len() {
        let cur_window = depths[i] + depths[i + 1] + depths[i + 2];
        if cur_window > last_window {
            increase_count += 1;
        }
        last_window = cur_window;
        i += 1;
    }

    increase_count
}
