use aoc_runner_derive::*;

type Image = Vec<Vec<u8>>;

#[derive(Debug)]
struct Input {
  algorithm: Vec<u8>,
  image: Image,
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
  let mut lines = input.lines();
  let algorithm = lines
    .next()
    .unwrap()
    .chars()
    .map(|c| match c {
      '#' => 1,
      _ => 0,
    })
    .collect();
  let image = lines
    .skip(1)
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => 1,
          _ => 0,
        })
        .collect()
    })
    .collect();
  Input { algorithm, image }
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> u32 {
  let mut image = input.image.clone();
  for iter in 0..2 {
    image = enhance(&image, &input.algorithm, iter);
  }
  print_image(&image);
  count_ones(&image)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> u32 {
  let mut image = input.image.clone();
  for iter in 0..50 {
    image = enhance(&image, &input.algorithm, iter);
  }
  count_ones(&image)
}

fn enhance(image: &Image, algorithm: &Vec<u8>, iter: usize) -> Image {
  let new_image_size = image.len() + 2;
  let mut new_image = vec![vec![0u8; new_image_size]; new_image_size];
  for row in 0..new_image_size {
    for col in 0..new_image_size {
      let row_old = row as i32 - 1;
      let col_old = col as i32 - 1;
      let index = get_grid_value(image, row_old - 1, col_old - 1, iter) * 256
        + get_grid_value(image, row_old - 1, col_old, iter) * 128
        + get_grid_value(image, row_old - 1, col_old + 1, iter) * 64
        + get_grid_value(image, row_old, col_old - 1, iter) * 32
        + get_grid_value(image, row_old, col_old, iter) * 16
        + get_grid_value(image, row_old, col_old + 1, iter) * 8
        + get_grid_value(image, row_old + 1, col_old - 1, iter) * 4
        + get_grid_value(image, row_old + 1, col_old, iter) * 2
        + get_grid_value(image, row_old + 1, col_old + 1, iter);

      if row == 0 && col == 0 {
        println!("get_grid_value(image, row_old, col_old) = {}", get_grid_value(image, row_old, col_old, iter));
        println!("row_old = {}, col_old = {}", row_old, col_old);
        println!("algorithm[{}] = {}", index, algorithm[index]);
      }

      new_image[row][col] = algorithm[index];
    }
  }
  new_image
}

fn get_grid_value(image: &Image, row: i32, col: i32, iter: usize) -> usize {
  if col < 0 || col >= image.len() as i32 || row < 0 || row >= image.len() as i32 {
    if iter % 2 == 1 { 1 } else { 0 }
  } else {
    image[row as usize][col as usize] as usize
  }
}

fn print_image(image: &Image) {
  for row in image {
    for cell in row {
      if *cell == 1 {
        print!("#")
      } else {
        print!(".")
      }
    }
    println!();
  }
  println!();
}

fn count_ones(image: &Image) -> u32 {
  image.iter().flatten().map(|n| *n as u32).sum()
}
