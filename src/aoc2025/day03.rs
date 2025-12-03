//! # Day 3: Lobby
//!
//! The input is lines of digits. Picking two digits from a line and combining
//! them while keeping their order gives a possible two-digit rating for that
//! line. The solution is the sum of the largest possible ratings of all lines.
//!
//! [puzzle site](https://adventofcode.com/2025/day/2)

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit = 0;
        let mut second_digit = 0;
        let mut number = 0;
        for char in line.chars() {
            let digit = char.to_digit(10).ok_or("invalid character")?;
            if digit > second_digit {
                second_digit = digit;
                number = first_digit * 10 + second_digit;
            }
            if digit > first_digit {
                second_digit = 0;
                first_digit = digit;
            }
        }
        sum += number;
    }
    Ok(sum.to_string())
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
}
