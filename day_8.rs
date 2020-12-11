use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

fn main() {
    let mut instructions = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();
    let first_answer = evaluate_instructions(&instructions).unwrap_err();
    println!("first answer: {}", first_answer);
    let second_answer = fix_and_evaluate_instructions(&mut instructions).unwrap();
    println!("second answer: {}", second_answer);
}

fn fix_and_evaluate_instructions(instructions: &mut Vec<Instruction>) -> Result<isize, ()> {
    for i in 0..instructions.len() {
        if !instructions[i].fix() {
            continue
        }
        match evaluate_instructions(instructions) {
            Ok(acc) => return Ok(acc),
            _ => {}
        }
        instructions[i].fix();
    }
    Err(())
}

fn evaluate_instructions(instructions: &Vec<Instruction>) -> Result<isize, isize> {
    let mut visited = HashSet::new();
    let mut index = 0;
    let mut acc: isize = 0;
    while index < instructions.len() {
        if visited.contains(&index) {
            return Err(acc);
        }
        visited.insert(index);
        match instructions[index] {
            Instruction::Acc(a) => { acc += a },
            Instruction::Jmp(a) => { index = (index as isize + a) as usize - 1 },
            _ => {},
        }
        index += 1;
    }
    Ok(acc)
}

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn fix(self: &mut Self) -> bool {
        *self = match self {
            Self::Jmp(a) => Self::Nop(*a),
            Self::Nop(a) => Self::Jmp(*a),
            _ => return false
        };
        true
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    UnknownInstruction(String),
    MissingSign,
    UnknownSign(char),
    InvalidOffset(ParseIntError),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let instruction_name = parts.next().unwrap();
        let offset_str = match parts.next() {
            Some(s) => s,
            _ => return Err(Self::Err::MissingSign)
        };
        let sign_char = match offset_str.chars().nth(0) {
            Some(c) => c,
            _ => return Err(Self::Err::MissingSign)
        };
        let offset_num = match offset_str[1..].parse::<isize>() {
            Ok(num) => num,
            Err(e) => return Err(Self::Err::InvalidOffset(e))
        };
        let sign = match sign_char {
            '+' => 1,
            '-' => -1,
            _ => return Err(Self::Err::UnknownSign(sign_char)),
        };
        let offset = sign * offset_num;
        Ok(match instruction_name {
            "acc" => Instruction::Acc(offset),
            "jmp" => Instruction::Jmp(offset),
            "nop" => Instruction::Nop(offset),
            _ => return Err(Self::Err::UnknownInstruction(instruction_name.to_string()))
        })
    }
}
