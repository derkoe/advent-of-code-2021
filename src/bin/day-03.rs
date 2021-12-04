use std::fs;

fn main() {
    let contents = fs::read_to_string("input-03.txt").unwrap();
    let lines = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

fn part_one(lines: &Vec<Vec<char>>) -> usize {
    get_rate(lines, true) * get_rate(lines, false)
}

fn part_two(lines: &Vec<Vec<char>>) -> usize {
    let m = most_common(lines, 1, true);
    lines.iter().filter(|line| line[1] != m).count()
    // TODO
}

fn get_rate(lines: &Vec<Vec<char>>, epsilon: bool) -> usize {
    let mut result = String::new();
    let line_length = lines[0].len();

    for n in 0..line_length {
        result += most_common(lines, n, epsilon).to_string().as_str();
    }

    usize::from_str_radix(result.as_str(), 2).unwrap()
}

fn most_common(lines: &Vec<Vec<char>>, n: usize, inverse: bool) -> char {
    let mut one_count = 0;
    let mut zero_count = 0;
    for line in 0..lines.len() {
        match lines[line][n] {
            '1' => one_count += 1,
            '0' => zero_count += 1,
            _ => (),
        }
    }
    if one_count > zero_count {
        if inverse {
            '0'
        } else {
            '1'
        }
    } else {
        if inverse {
            '1'
        } else {
            '0'
        }
    }
}
