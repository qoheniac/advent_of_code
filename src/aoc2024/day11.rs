//! # Day 11: Plutonian Pebbles
//!
//! The input contains a few numbers separated by spaces. On each iteration
//! zeros are replaced by ones, numbers with an even number of digits are split
//! in half to become two numbers, and all others are multiplied by 2024.
//!
//! [puzzle site](https://adventofcode.com/2024/day11)

fn solution(input: String, iterations: u8) -> crate::PuzzleResult {
    let mut numbers: Vec<u64> =
        (input.split_whitespace().map(|n| n.parse())).collect::<Result<_, _>>()?;
    let mut counts = vec![1u64; numbers.len()];
    for _ in 0..iterations {
        let mut i = 0;
        while i < numbers.len() {
            let number = numbers[i];
            if number == 0 {
                numbers[i] = 1;
            } else if number.ilog10() % 2 != 0 {
                let n = (number.ilog(10) + 1) / 2;
                numbers[i] = number / 10u64.pow(n);
                numbers.insert(i + 1, number - numbers[i] * 10u64.pow(n));
                counts.insert(i + 1, counts[i]);
                i += 1;
            } else {
                numbers[i] *= 2024;
            }
            i += 1;
        }
        i = 0;
        while i < numbers.len() {
            let mut j = i + 1;
            while j < numbers.len() {
                if numbers[j] == numbers[i] {
                    numbers.remove(j);
                    counts[i] += counts.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    Ok(counts.into_iter().sum::<u64>().to_string())
}

/// Part 1: 25 Iterations
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 25)
}

/// Part 2: 75 Iterations
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, 75)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "55312");
    }
}
