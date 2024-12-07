//! # Day 7: Bridge Repair
//!
//! Each input line holds a result followed by a colon and a bunch of numbers
//! that combined with operators left to right might produce the result. The
//! solution is the sum of all results for which this was possible.
//!
//! [puzzle site](https://adventofcode.com/2024/day07)

fn is_valid_equation(result: u64, numbers: Vec<u64>, ops: &[fn(u64, u64) -> u64]) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }
    for op in ops {
        let mut new_numbers = vec![op(numbers[0], numbers[1])];
        new_numbers.extend_from_slice(&numbers[2..]);
        if new_numbers[0] <= result && is_valid_equation(result, new_numbers, ops) {
            return true;
        }
    }
    false
}

fn solution(input: String, ops: &[fn(u64, u64) -> u64]) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").ok_or("no colon found")?;
        let result = result.parse()?;
        let numbers = (numbers.split_whitespace().map(|s| s.parse())).collect::<Result<_, _>>()?;
        if is_valid_equation(result, numbers, ops) {
            sum += result;
        }
    }
    Ok(sum.to_string())
}

/// Part 1: Combinations of addition and multiplication
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, &[|a, b| a * b, |a, b| a + b])
}

fn concatenation(a: u64, b: u64) -> u64 {
    10u64.pow(1 + b.ilog10()) * a + b
}

/// Part 2: Combinations of addition, multiplication, and concatenation
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, &[|a, b| a * b, |a, b| a + b, concatenation])
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "190: 10 19\n",
        "3267: 81 40 27\n",
        "83: 17 5\n",
        "156: 15 6\n",
        "7290: 6 8 6 15\n",
        "161011: 16 10 13\n",
        "192: 17 8 14\n",
        "21037: 9 7 18 13\n",
        "292: 11 6 16 20"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "3749");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "11387");
    }
}
