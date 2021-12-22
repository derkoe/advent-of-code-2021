use aoc_runner_derive::*;

#[aoc(day21, part1)]
fn part1(_input: &str) -> usize {
  let mut pos_player_1 = 6;
  let mut pos_player_2 = 2;
  let mut points_player_1 = 0;
  let mut points_player_2 = 0;
  let mut throws = 0;
  loop {
    // player 1
    pos_player_1 =
      (pos_player_1 + dice_value(throws + 1) + dice_value(throws + 2) + dice_value(throws + 3))
        % 10;
    pos_player_1 = if pos_player_1 == 0 { 10 } else { pos_player_1 };
    points_player_1 += pos_player_1;
    // println!("Player 1 rolls {}+{}+{} and moves to space {} for a total score of {}", dice_value(throws + 1), dice_value(throws + 2), dice_value(throws + 3), pos_player_1, points_player_1);
    throws += 3;
    if points_player_1 >= 1000 {
      println!("Player 1 wins!");
      return throws * points_player_2;
    }
    // player 2
    pos_player_2 =
      (pos_player_2 + dice_value(throws + 1) + dice_value(throws + 2) + dice_value(throws + 3))
        % 10;
    pos_player_2 = if pos_player_2 == 0 { 10 } else { pos_player_2 };
    throws += 3;
    points_player_2 += pos_player_2;
    if points_player_2 >= 1000 {
      println!("Player 2 wins!");
      return throws * points_player_1;
    }
  }
}

fn dice_value(throw: usize) -> usize {
  throw % 100
}

#[aoc(day21, part2)]
fn part2(_input: &str) -> usize {
  const THROWS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
  let mut pos_player_1 = 6;
  let mut pos_player_2 = 2;
  let mut points_player_1 = 0;
  let mut points_player_2 = 0;
  let mut throws = 0;
  // TODO
  0
}
