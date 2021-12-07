use aoc_runner_derive::*;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<u8> {
    input.split(",").map(|x| x.parse::<u8>().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(depths: &Vec<u8>) -> usize {
    let mut curr_depths = depths.clone();
    for _ in 0..80 {
        let mut add = 0;
        curr_depths = curr_depths
            .iter()
            .map(|timer| {
                if *timer == 0 {
                    add += 1;
                    6
                } else {
                    timer - 1
                }
            })
            .collect();
        for _ in 0..add {
            curr_depths.push(8);
        }
    }
    curr_depths.len()
}

#[aoc(day6, part2)]
fn part2(timers: &Vec<u8>) -> usize {
    let mut counts = [0; 9];
    for timer in timers {
        counts[*timer as usize] += 1;
    }
    for _ in 0..256 {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum::<usize>()
}
