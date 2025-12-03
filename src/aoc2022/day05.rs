//! # Day 5: Supply Stacks
//!
//! [puzzle site](https://adventofcode.com/2022/day/5)

enum Part {
    Part1,
    Part2,
}
use Part::*;

fn solution(input: String, part: Part) -> crate::PuzzleResult {
    let mut blocks = input.split("\n\n");

    // parse starting stacks
    let starting_stacks = blocks.next().unwrap();
    let mut starting_rows = starting_stacks.lines().rev();
    let numbers_row = starting_rows.next().unwrap();
    let mut stack_indices = Vec::new();
    let mut stacks = Vec::new();
    for (index, character) in numbers_row.chars().enumerate() {
        if character.is_numeric() {
            stack_indices.push(index);
            stacks.push(Vec::new());
        }
    }
    for row in starting_rows {
        let row_chars: Vec<char> = row.chars().collect();
        for i in 0..stack_indices.len() {
            let character = row_chars[stack_indices[i]];
            if character.is_alphabetic() {
                stacks[i].push(character);
            }
        }
    }

    // parse and do rearrangements
    let rearrangements = blocks.next().unwrap();
    for rearrangement in rearrangements.lines() {
        let words: Vec<&str> = rearrangement.split_whitespace().collect();
        let number: usize = words[1].parse().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let dest = words[5].parse::<usize>().unwrap() - 1;
        match part {
            Part1 => {
                for _ in 0..number {
                    let crt = stacks[from].pop().unwrap();
                    stacks[dest].push(crt);
                }
            }
            Part2 => {
                let len = stacks[from].len();
                let mut crates = stacks[from].split_off(len - number);
                stacks[dest].append(&mut crates);
            }
        }
    }

    // costruct output
    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.last().unwrap());
    }
    Ok(output.to_string())
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Part1)
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, Part2)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "MCD");
    }
}
