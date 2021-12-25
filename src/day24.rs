use aoc_runner_derive::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Var {
    W,
    X,
    Y,
    Z,
}

impl Var {
    fn from_string(s: &str) -> Var {
        match s {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => panic!("Invalid var: {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Arg {
    Value(i32),
    Variable(Var),
    None,
}

#[derive(Debug, Copy, Clone)]
enum InstructionType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eq,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    instr: InstructionType,
    arg1: Arg,
    arg2: Arg,
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            if split.len() == 2 {
                Instruction {
                    instr: InstructionType::Inp,
                    arg1: Arg::Variable(Var::from_string(split[1])),
                    arg2: Arg::None,
                }
            } else {
                let instr = match split[0] {
                    "add" => InstructionType::Add,
                    "mul" => InstructionType::Mul,
                    "div" => InstructionType::Div,
                    "mod" => InstructionType::Mod,
                    "eql" => InstructionType::Eq,
                    _ => panic!("Unknown instruction: {}", split[0]),
                };
                let arg1 = split[1]
                    .parse()
                    .map(|val| Arg::Value(val))
                    .unwrap_or_else(|_| Arg::Variable(Var::from_string(split[1])));
                let arg2 = split[2]
                    .parse()
                    .map(|val| Arg::Value(val))
                    .unwrap_or_else(|_| Arg::Variable(Var::from_string(split[2])));
                Instruction { instr, arg1, arg2 }
            }
        })
        .collect()
}

fn op(instr: InstructionType, arg1: i32, arg2: i32) -> i32 {
    match instr {
        InstructionType::Add => arg1 + arg2,
        InstructionType::Mul => arg1 * arg2,
        InstructionType::Div => arg1 / arg2,
        InstructionType::Mod => arg1 % arg2,
        InstructionType::Eq => {
            if arg1 == arg2 {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown instruction: {:?}", instr),
    }
}

fn compute(model_number: &str, instructions: &Vec<Instruction>) -> i32 {
    let mut vars: HashMap<&Var, i32> = HashMap::new();
    let mut chars = model_number.chars();
    for instruction in instructions {
        match instruction.instr {
            InstructionType::Inp => match instruction.arg1 {
                Arg::Variable(ref var) => {
                    let value = chars.next().unwrap().to_digit(10).unwrap();
                    vars.insert(var, value as i32);
                }
                _ => panic!("Expected variable"),
            },
            _ => match instruction.arg1 {
                Arg::Variable(ref var1) => match instruction.arg2 {
                    Arg::Variable(ref var2) => {
                        let val1 = vars.get(&var1).unwrap_or(&0);
                        let val2 = vars.get(&var2).unwrap_or(&0);
                        vars.insert(var1, op(instruction.instr, *val1, *val2));
                    }
                    Arg::Value(val2) => {
                        let val1 = vars.get(&var1).unwrap_or(&0);
                        vars.insert(var1, op(instruction.instr, *val1, val2));
                    }
                    _ => panic!("Second argument must be a variable or value"),
                },
                _ => panic!("First argument must be a variable"),
            },
        }
    }
    *vars.get(&Var::Z).unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &Vec<Instruction>) -> u64 {
    for i in (11111111111111..99999999999999u64).rev() {
        let x = format!("{}", i);
        if x.contains("0") {
            continue;
        }
        if compute(&x, input) == 0 {
            return i;
        }
    }
    0
}

#[aoc(day24, part2)]
fn part2(input: &Vec<Instruction>) -> u64 {
    for i in 11111111111111..99999999999999u64 {
        let x = format!("{}", i);
        if x.contains("0") {
            continue;
        }
        if compute(&x, input) == 0 {
            return i;
        }
    }
    0
}
