use aoc_runner_derive::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    map: HashMap<(char, char), char>,
    pairs: HashMap<(char, char), usize>,
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Input {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        if line.contains("->") {
            let (from, to) = line.split_once(" -> ").unwrap();
            let mut from_chars = from.chars();
            map.insert(
                (from_chars.next().unwrap(), from_chars.next().unwrap()),
                to.chars().next().unwrap(),
            );
        }
    });

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let polymer: Vec<char> = input.lines().next().unwrap().chars().collect();
    for i in 0..polymer.len() - 1 {
        pairs.insert((polymer[i], polymer[i + 1]), 1);
    }

    Input { map, pairs }
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> usize {
    calc(input, 10)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> usize {
    calc(input, 40)
}

fn calc(input: &Input, generations: usize) -> usize {
    let mut pairs = input.pairs.clone();
    for _gen in 1..=generations {
        println!("Gen {}", _gen);
        let old_pairs = pairs.clone();
        println!("{:?}", pairs);
        pairs = HashMap::new();
        for (pair, count) in old_pairs.iter() {
            match input.map.get(pair) {
                Some(insert) => {
                    *pairs.entry((pair.0, *insert)).or_insert(0) += count;
                    *pairs.entry((*insert, pair.1)).or_insert(0) += count;
                }
                None => continue,
            }
        }
    }

    let mut counter = HashMap::new();
    for pair in pairs {
        *counter.entry(pair.0 .0).or_insert(0) += pair.1;
        *counter.entry(pair.0 .1).or_insert(0) += pair.1;
    }

    println!("{:?}", counter);

    let (min, max) = counter
        .into_iter()
        .fold((100000000, 0), |mut acc, (_k, v)| {
            acc.0 = acc.0.min(v / 2);
            acc.1 = acc.1.max(v / 2);
            acc
        });

    max - min
}
