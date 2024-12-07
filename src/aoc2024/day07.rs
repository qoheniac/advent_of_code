//! # Day 7: Bridge Repair
//!
//! Each input line holds a result followed by a colon and a bunch of numbers
//! that added and/or multiplied left to right might produce the result.
//!
//! [puzzle site](https://adventofcode.com/2024/day07)

fn is_valid_equation(result: u64, numbers: Vec<u64>) -> bool {
    if numbers.len() == 1 {
        return result == numbers[0];
    }
    for op in [|a, b| a * b, |a, b| a + b] {
        let mut new_numbers = vec![op(numbers[0], numbers[1])];
        new_numbers.extend_from_slice(&numbers[2..]);
        if new_numbers[0] <= result && is_valid_equation(result, new_numbers) {
            return true;
        }
    }
    false
}

/// Part 1: Sum of all results that can be produced from the numbers next to it
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").ok_or("no colon found")?;
        let result = result.parse()?;
        let numbers = (numbers.split_whitespace().map(|s| s.parse())).collect::<Result<_, _>>()?;
        if is_valid_equation(result, numbers) {
            sum += result;
        }
    }
    Ok(sum.to_string())
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
}
