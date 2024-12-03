//! # Day 3: Mull It Over
//!
//! The input is one line holding a bunch of instructions of the form mul(X,Y)
//! surrounded by other characters that can be ignored. The integer numbers X
//! and Y have to be multiplied.
//!
//! [puzzle site](https://adventofcode.com/2024/day/3)

use regex::Regex;

/// Part 1: Sum of all Products
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

#[cfg(test)]
mod tests {
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "161");
    }
}
