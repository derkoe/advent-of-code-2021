use std::fs;

fn main() {
    let contents = fs::read_to_string("input-01.txt").expect("Could not read input-01.txt");
    let depths: Vec<u16> = contents.split("\n")
        .map(|x| u16::from_str_radix(x, 10).unwrap_or(0))
        .collect();

    println!("Part 1: {} times increased", part1(&depths));

    println!("Part 2: {} times increased", part2(&depths));
}

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

fn part2(depths: &Vec<u16>) -> u16 {
    let mut increase_count = 0;
    let mut last_window = u16::max_value();
    
    let mut i = 0;
    while i + 2 < depths.len() {
        let cur_window = depths[i] + depths[i+1] + depths[i+2];
        if cur_window > last_window {
            increase_count += 1;
        }
        last_window = cur_window;
        i += 1;
    }

    increase_count
}