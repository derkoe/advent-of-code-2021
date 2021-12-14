use aoc_runner_derive::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    map: HashSet<(usize, usize)>,
    folds: Vec<Fold>,
}

#[derive(Debug)]
struct Fold {
    direction: Direction,
    position: usize,
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Input {
    let mut map = HashSet::new();
    let mut folds = vec![];
    input.lines().for_each(|line| {
        if line.starts_with("fold") {
            let (dir, pos) = line[11..].split_once("=").unwrap();
            folds.push(Fold {
                direction: match dir {
                    "x" => Direction::X,
                    "y" => Direction::Y,
                    _ => panic!("Unknown direction"),
                },
                position: pos.parse::<usize>().unwrap(),
            });
        } else if !line.is_empty() {
            let (x, y) = line
                .split_once(",")
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap();
            map.insert((x, y));
        }
    });
    Input { map, folds }
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> usize {
    let mut map = input.map.clone();
    fold(&mut map, &input.folds[0]);
    map.len()
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> usize {
    let mut map = input.map.clone();
    input.folds.iter().for_each(|f| fold(&mut map, f));

    let mut max_x = 0;
    let mut max_y = 0;
    map.iter().for_each(|(x, y)| {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    });
    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    0
}

fn fold(map: &mut HashSet<(usize, usize)>, fold: &Fold) {
    match fold.direction {
        Direction::X => {
            for (x, y) in map.clone().iter() {
                if *x > fold.position {
                    map.insert((x - 2 * (x - fold.position), *y));
                    map.remove(&(*x, *y));
                }
            }
        }
        Direction::Y => {
            for (x, y) in map.clone().iter() {
                if *y > fold.position {
                    map.insert((*x, y - 2 * (y - fold.position)));
                    map.remove(&(*x, *y));
                }
            }
        }
    }
}
