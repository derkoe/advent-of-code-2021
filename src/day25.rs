use aoc_runner_derive::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    size_x: usize,
    size_y: usize,
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Map {
    let lines = input.lines();
    let mut map = Map {
        size_x: lines.clone().map(|l| l.len()).max().unwrap(),
        size_y: lines.clone().count(),
        east: HashSet::new(),
        south: HashSet::new(),
    };
    lines.enumerate().for_each(|(line_no, line)| {
        line.chars()
            .enumerate()
            .for_each(|(col_no, c)| {
                if c == '>' {
                    map.east.insert((col_no, line_no));
                } else if c == 'v' {
                    map.south.insert((col_no, line_no));
                }
            });
    });
    map
}

#[aoc(day25, part1)]
fn part2(input: &Map) -> usize {
    let mut east = input.east.clone();
    let mut south = input.south.clone();
    for i in 1..50_000_000 {
        let mut moves = 0;
        let mut new_east = HashSet::new();
        for (col, line) in east.iter() {
            let next_x = (col + 1) % input.size_x;
            if east.contains(&(next_x, *line)) || south.contains(&(next_x, *line)) {
                new_east.insert((*col, *line));
            } else {
                new_east.insert((next_x, *line));
                moves += 1;
            }
        }
        east = new_east;
        let mut new_south = HashSet::new();
        for (col, line) in south.iter() {
            let next_y = (line + 1) % input.size_y;
            if east.contains(&(*col, next_y)) || south.contains(&(*col, next_y)) {
                new_south.insert((*col, *line));
            } else {
                new_south.insert((*col, next_y));
                moves += 1;
            }
        }
        south = new_south;
        if moves == 0 {
            return i;
        }
    }
    0
}
