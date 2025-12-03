//! # Day 3: Lobby
//!
//! The input is lines of digits. Picking a certain number of digits from a line
//! and concatenating in order gives a possible rating for that line. The
//! solution is the sum of the largest possible ratings of all lines.
//!
//! [puzzle site](https://adventofcode.com/2025/day/2)

fn solution(input: String, digits_count: usize) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let mut bests = vec![0; digits_count];
        for char in line.chars() {
            let digit = char.to_digit(10).ok_or("invalid character")? as u64;
            let mut carry = 0;
            for i in 0..digits_count {
                let candidate = carry * 10 + digit;
                carry = bests[i];
                if candidate > bests[i] {
                    bests[i] = candidate;
                }
            }
        }
        sum += bests[digits_count - 1];
    }
    Ok(sum.to_string())
}

/// Part 1: Two digits per line
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 2)
}

/// Part 2: Twelve digits per line
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, 12)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "987654321111111\n",
        "811111111111119\n",
        "234234234234278\n",
        "818181911112111"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "357");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "3121910778619");
    }
}
