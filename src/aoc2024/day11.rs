//! # Day 11: Plutonian Pebbles
//!
//! The input contains a few numbers separated by spaces. On each iteration
//! zeros are replaced by ones, numbers with an even number of digits are split
//! in half to become two numbers, and all others are multiplied by 2024.
//!
//! [puzzle site](https://adventofcode.com/2024/day11)

/// 25 Iterations
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut numbers: Vec<u64> =
        (input.split_whitespace().map(|n| n.parse())).collect::<Result<_, _>>()?;
    for _ in 0..25 {
        let mut i = 0;
        while i < numbers.len() {
            let number = numbers[i];
            if number == 0 {
                numbers[i] = 1;
            } else if number.ilog10() % 2 != 0 {
                let n = (number.ilog(10) + 1) / 2;
                numbers[i] = number / 10u64.pow(n);
                numbers.insert(i + 1, number - numbers[i] * 10u64.pow(n));
                i += 1;
            } else {
                numbers[i] *= 2024;
            }
            i += 1;
        }
    }
    Ok(numbers.len().to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "55312");
    }
}
