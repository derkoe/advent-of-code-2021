use aoc_runner_derive::*;

#[derive(Debug)]
struct RebootStep {
    switch_on: u8,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<RebootStep> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(&[' ', ','][..]).collect();
            RebootStep {
                switch_on: if split[0] == "on" { 1 } else { 0 },
                x: parse_coord(split[1]),
                y: parse_coord(split[2]),
                z: parse_coord(split[3]),
            }
        })
        .collect()
}

fn parse_coord(s: &str) -> (i32, i32) {
    s[2..]
        .split_once("..")
        .map(|(first, second)| {
            (
                first.parse::<i32>().unwrap(),
                second.parse::<i32>().unwrap(),
            )
        })
        .unwrap()
}

#[aoc(day22, part1)]
fn part1(input: &Vec<RebootStep>) -> u32 {
    let mut space = [[[0u8; 100]; 100]; 100];
    for step in input {
        if step.x.0 >= -50
            && step.x.1 <= 50
            && step.y.0 >= -50
            && step.y.1 <= 50
            && step.z.0 >= -50
            && step.z.1 <= 50
        {
            for x in step.x.0..=step.x.1 {
                for y in step.y.0..=step.y.1 {
                    for z in step.z.0..=step.z.1 {
                        space[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] =
                            step.switch_on;
                    }
                }
            }
        }
    }

    let mut sum = 0u32;
    for x in space {
        for y in x {
            for z in y {
                sum += z as u32;
            }
        }
    }

    sum
}

#[aoc(day22, part2)]
fn part2(input: &Vec<RebootStep>) -> usize {
    let mut cubes: Vec<Cube> = Vec::new();
    for step in input {
        let cube = Cube::new(step.x, step.y, step.z);
        cubes.iter_mut().for_each(|x| x.subtract(&cube));
        if step.switch_on == 1 {
            cubes.push(cube);
        }
    }
    cubes.iter().map(|x| x.volume() as usize).sum::<usize>()
}

fn intersects(a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 <= b.0 && b.0 <= a.1)
        || (a.0 <= b.1 && b.1 <= a.1)
        || (b.0 <= a.0 && a.0 <= b.1)
        || (b.0 <= a.1 && a.1 <= b.1)
}

fn intersection(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (
        if b.0 < a.0 { a.0 } else { b.0 },
        if b.1 < a.1 { b.1 } else { a.1 },
    )
}

struct Cube {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),

    off: Vec<Cube>,
}

impl Cube {
    pub fn new(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Self {
        return Cube {
            x: x,
            y: y,
            z: z,

            off: Vec::new(),
        };
    }

    fn is_intresecting(&self, other: &Cube) -> bool {
        intersects(self.x, other.x) && intersects(self.y, other.y) && intersects(self.z, other.z)
    }

    pub fn subtract(&mut self, other: &Cube) {
        if self.is_intresecting(other) {
            let x = intersection(self.x, other.x);
            let y = intersection(self.y, other.y);
            let z = intersection(self.z, other.z);

            self.off.iter_mut().for_each(|x| x.subtract(other));
            self.off.push(Cube::new(x, y, z));
        }
    }

    pub fn volume(&self) -> u128 {
        ((self.x.1 - self.x.0 + 1) as u128
            * (self.y.1 - self.y.0 + 1) as u128
            * (self.z.1 - self.z.0 + 1) as u128)
            - self.off.iter().map(|x| x.volume()).sum::<u128>()
    }
}
