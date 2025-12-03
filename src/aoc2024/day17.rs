//! # Day 17: Chronospatial Computer
//!
//! The input has the following form
//!
//! ```text
//! Register A: {a}
//! Register B: {b}
//! Register C: {c}
//!
//! Program: {program}
//! ```
//!
//! describing a computer with three registers initially holding the numbers
//! `a`, `b`, and `c` as well as a program which is a comma-separated list of
//! opcodes and operands in alternation both being numbers smaller than 8.
//!
//! For details about the opcodes, either see the
//! [puzzle site](https://adventofcode.com/2024/day17) or the
//! [implementation](../../../src/advent_of_code/aoc2024/day17.rs.html).

use std::{error::Error, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Adv,
            "1" => Bxl,
            "2" => Bst,
            "3" => Jnz,
            "4" => Bxc,
            "5" => Out,
            "6" => Bdv,
            "7" => Cdv,
            _ => Err(format!("invalid opcode: {s}"))?,
        })
    }
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<(Instruction, usize)>,
    pointer: usize,
}

impl Computer {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            o if o < 4 => o,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand"),
        }
    }

    fn run(&mut self) -> String {
        let mut output = String::new();
        while self.pointer < self.program.len() {
            let (instruction, operand) = self.program[self.pointer];
            self.pointer += 1;
            match instruction {
                Adv => self.a /= 1 << self.combo(operand),
                Bxl => self.b ^= operand,
                Bst => self.b = self.combo(operand) & 7,
                Jnz if self.a == 0 => (),
                Jnz => self.pointer = operand >> 1,
                Bxc => self.b ^= self.c,
                Out if output.is_empty() => output = (self.combo(operand) & 7).to_string(),
                Out => output.push_str(&format!(",{}", self.combo(operand) & 7)),
                Bdv => self.b = self.a / (1 << self.combo(operand)),
                Cdv => self.c = self.a / (1 << self.combo(operand)),
            }
        }
        output
    }
}

fn parse_register(line: Option<&str>, register: char) -> Result<usize, Box<dyn Error>> {
    Ok(line
        .and_then(|line| line.strip_prefix(&format!("{register}: ")))
        .ok_or(format!("register {register} not found"))?
        .parse()?)
}

impl FromStr for Computer {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, code) = s.split_once("\n\n").ok_or("blank line missing")?;
        let mut registers = (registers.lines()).flat_map(|line| line.strip_prefix("Register "));
        let a = parse_register(registers.next(), 'A')?;
        let b = parse_register(registers.next(), 'B')?;
        let c = parse_register(registers.next(), 'C')?;
        let mut program = Vec::new();
        let code = (code.trim().strip_prefix("Program: ")).ok_or("program missing")?;
        for mut command in &code.split(",").chunks(2) {
            program.push((
                command.next().unwrap().parse()?,
                command.next().ok_or("uneven program length")?.parse()?,
            ));
        }
        Ok(Self {
            a,
            b,
            c,
            program,
            pointer: 0,
        })
    }
}

/// Part 1: Find the output of the program
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut computer: Computer = input.parse()?;
    Ok(computer.run())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "Register A: 729\n",
        "Register B: 0\n",
        "Register C: 0\n",
        "\n",
        "Program: 0,1,5,4,3,0",
    );

    #[test]
    fn test_part1() {
        assert_eq!(
            &super::part1(INPUT.to_string()).unwrap(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
