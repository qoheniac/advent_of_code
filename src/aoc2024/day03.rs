//! # Day 3: Mull It Over
//!
//! The input is one line holding a bunch of instructions of the form mul(X,Y),
//! do(), or don't() surrounded by other characters that can be ignored. The
//! integer numbers X and Y have to be multiplied but only if the last do or
//! don't instruction was do().
//!
//! [puzzle site](https://adventofcode.com/2024/day/3)

use regex::Regex;

/// Part 1: Sum of all Products ignoring do and don't
pub fn part1(input: String) -> crate::PuzzleResult {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
        let x: u32 = x.parse()?;
        let y: u32 = y.parse()?;
        sum += x * y;
    }
    Ok(sum.to_string())
}

/// Part 2: Sum of all Products respecting do and don't
pub fn part2(input: String) -> crate::PuzzleResult {
    let re = Regex::new(r"(mul\((?<x>[0-9]+),(?<y>[0-9]+)\))|(?<do>do(?<not>n't)?\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for caps in re.captures_iter(&input) {
        if caps.name("do").is_some() {
            if caps.name("not").is_some() {
                enabled = false;
            } else {
                enabled = true;
            }
        } else if enabled {
            let x: u32 = caps.name("x").unwrap().as_str().parse()?;
            let y: u32 = caps.name("y").unwrap().as_str().parse()?;
            sum += x * y;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(&super::part1(input.to_string()).unwrap(), "161");
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(&super::part2(input.to_string()).unwrap(), "48");
    }
}
