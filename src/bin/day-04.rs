use std::fs;
use std::str::Lines;

type Board = Vec<Vec<(u32, bool)>>;

fn main() {
    println!(
        "Part 1: score = {}",
        part1(parse_file(fs::read_to_string("input-04.txt").unwrap()))
    );

    println!(
        "Part 2: score = {}",
        part2(parse_file(fs::read_to_string("input-04.txt").unwrap()))
    );
}

fn part1(input: (Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, mut boards) = input.clone();
    for num in numbers {
        for board in boards.iter_mut() {
            if check_board(num, board) {
                return calc_board(board.clone()) * num;
            }
        }
    }
    0
}

fn part2(input: (Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, mut boards) = input.clone();
    let mut last_board = Option::None;
    let mut last_num = 0;
    for num in numbers {
        let mut winner_boards = vec![];
        for (idx, board) in boards.iter_mut().enumerate() {
            if check_board(num, board) {
                winner_boards.push(idx);
                last_board = Option::from(board.clone());
                last_num = num.clone();
            }
        }
        winner_boards.iter().rev().for_each(|&idx| {
            boards.remove(idx);
        });
    }
    calc_board(last_board.unwrap()) * last_num
}

fn parse_file(contents: String) -> (Vec<u32>, Vec<Board>) {
    let mut lines = contents.lines();
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    lines.next();
    (numbers, parse_boards(&mut lines))
}

fn parse_boards(lines: &mut Lines) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = vec![];
    for line in lines {
        if line.trim().is_empty() {
            boards.push(board);
            board = vec![];
            continue;
        }
        board.push(
            line.split_whitespace()
                .map(|item| (item.parse::<u32>().unwrap(), false))
                .collect(),
        );
    }
    boards.push(board);
    boards
}

fn check_board(num: u32, board: &mut Board) -> bool {
    for line in board.iter_mut() {
        let mut row = true;
        for cell in line.iter_mut() {
            if cell.0 == num {
                cell.1 = true;
            }
            row = row && cell.1;
        }
        if row {
            return true;
        }
    }
    for col_idx in 0..board[0].len() {
        let mut col = true;
        for row_idx in 0..board.len() {
            col = col && board[row_idx][col_idx].1;
        }
        if col {
            return true;
        }
    }
    false
}

fn calc_board(board: Board) -> u32 {
    let mut sum = 0;
    for line in board.iter() {
        for cell in line.iter() {
            if !cell.1 {
                sum += cell.0;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::{parse_file, part1, part2};

    const EXAMPLE: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_part1() {
        assert_eq!(4512, part1(parse_file(String::from(EXAMPLE))));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924, part2(parse_file(String::from(EXAMPLE))));
    }
}
