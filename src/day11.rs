use aoc_runner_derive::*;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<u32>>) -> usize {
    let mut flashes = 0;
    let mut data = input.clone();
    for _gen in 1..101 {
        flashes += handle_gen(&mut data);
    }
    flashes
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<u32>>) -> usize {
    let mut data = input.clone();
    for gen in 1..1000 {
        if handle_gen(&mut data) == 100 {
            return gen;
        }
    }
    0
}

fn handle_gen(data: &mut Vec<Vec<u32>>) -> usize {
    let mut flashes = 0;
    data.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            *cell += 1;
        });
    });
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            handle_flash(data, row, col);
        }
    }
    data.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            if *cell > 9 {
                *cell = 0;
                flashes += 1;
            }
        });
    });
    flashes
}

fn handle_flash(data: &mut Vec<Vec<u32>>, row: usize, col: usize) {
    if data[row][col] != 10 {
        return;
    }

    data[row][col] += 1;

    if row + 1 < data.len() {
        if data[row + 1][col] <= 9 {
            data[row + 1][col] += 1;
            handle_flash(data, row + 1, col);
        }
        if col + 1 < data[row].len() && data[row + 1][col + 1] <= 9 {
            data[row + 1][col + 1] += 1;
            handle_flash(data, row + 1, col + 1);
        }
        if col > 0 && data[row + 1][col - 1] <= 9 {
            data[row + 1][col - 1] += 1;
            handle_flash(data, row + 1, col - 1);
        }
    }
    if row > 0 {
        if data[row - 1][col] <= 9 {
            data[row - 1][col] += 1;
            handle_flash(data, row - 1, col);
        }
        if col + 1 < data[row - 1].len() && data[row - 1][col + 1] <= 9 {
            data[row - 1][col + 1] += 1;
            handle_flash(data, row - 1, col + 1);
        }
        if col > 0 && data[row - 1][col - 1] <= 9 {
            data[row - 1][col - 1] += 1;
            handle_flash(data, row - 1, col - 1);
        }
    }
    if col + 1 < data[row].len() && data[row][col + 1] <= 9 {
        data[row][col + 1] += 1;
        handle_flash(data, row, col + 1);
    }
    if col > 0 && data[row][col - 1] <= 9 {
        data[row][col - 1] += 1;
        handle_flash(data, row, col - 1);
    }
}
