use aoc_runner_derive::*;

struct Command<'a> {
    command_type: &'a str,
    num: i32,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            Command {
                command_type: split[0],
                num: split[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_one(commands: &Vec<Command>) -> i32 {
    let mut depth = 0;
    let mut horizontal_position = 0;

    commands
        .iter()
        .for_each(|command| match command.command_type {
            "forward" => {
                horizontal_position += command.num;
            }
            "down" => {
                depth += command.num;
            }
            "up" => {
                depth -= command.num;
            }
            _ => {
                println!("Unknown command: {}", command.command_type);
            }
        });

    horizontal_position * depth
}

#[aoc(day2, part2)]
fn part_two(commands: &Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command.command_type {
            "forward" => {
                horizontal_position += command.num;
                depth += aim * command.num;
            }
            "down" => {
                aim += command.num;
            }
            "up" => {
                aim -= command.num;
            }
            _ => {
                println!("Unknown command: {}", command.command_type);
            }
        }
    }

    horizontal_position * depth
}
