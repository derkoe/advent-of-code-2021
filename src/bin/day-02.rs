use std::fs;

struct Command<'a> {
    command_type: &'a str,
    num: i32,
}

fn main() {
    let contents = fs::read_to_string("input-02.txt").unwrap();
    let commands = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            Command {
                command_type: split[0],
                num: split[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    println!(
        "Part 1: horizontal_position*depth = {}",
        part_one(&commands)
    );

    println!(
        "Part 1: horizontal_position*depth = {}",
        part_two(&commands)
    );
}

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
