use aoc_runner_derive::*;

#[derive(Debug)]
struct InputOutput {
    input: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day8)]
fn parse_input<'a>(input: &'a str) -> Vec<InputOutput> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(input, output)| InputOutput {
            input: input.split(" ").map(|s| String::from(s)).collect(),
            output: output.split(" ").map(|s| String::from(s)).collect(),
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<InputOutput>) -> usize {
    input
        .iter()
        .map(|io| {
            io.output
                .iter()
                .filter(|s| [2usize, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &Vec<InputOutput>) -> usize {
    let mut sum = 0;
    for line in input.iter() {
        let digits_of_1 = line
            .input
            .iter()
            .filter(|s| s.len() == 2)
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        let digits_of_4_without_1 = line
            .input
            .iter()
            .filter(|s| s.len() == 4)
            .next()
            .unwrap()
            .chars()
            .filter(|c| !digits_of_1.contains(c))
            .collect::<Vec<char>>();
        let num_str = line
            .output
            .iter()
            .map(|digit| {
                match digit.len() {
                    2 => "1",
                    3 => "7",
                    4 => "4",
                    7 => "8",
                    5 => {
                        // 2, 3 or 5
                        if contains_all(digit, &digits_of_1) {
                            "3"
                        } else if contains_all(digit, &digits_of_4_without_1) {
                            "5"
                        } else {
                            "2"
                        }
                    }
                    6 => {
                        // 0, 6 or 9
                        if contains_all(digit, &digits_of_4_without_1) {
                            if contains_all(digit, &digits_of_1) {
                                "9"
                            } else {
                                "6"
                            }
                        } else {
                            "0"
                        }
                    }
                    _ => "X",
                }
            })
            .collect::<String>();
        sum += num_str.parse::<usize>().unwrap();
    }
    sum
}

fn contains_all(s: &str, chars: &Vec<char>) -> bool {
    chars.iter().all(|c| s.contains(*c))
}
