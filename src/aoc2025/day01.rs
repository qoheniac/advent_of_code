//! # Day 1: Secret Entrance
//!
//! Rows hold numbers to add (R) or subtract (L) mod 100 starting at 50.
//!
//! [puzzle site](https://adventofcode.com/2025/day/1)

/// Part 1: Number of times 0 is encountered
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut number: i16 = 50;
    let mut count = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next().ok_or("empty line")?;
        let distance: i16 = chars.as_str().parse()?;
        number = (number
            + match direction {
                'R' => Ok(distance),
                'L' => Ok(-distance),
                _ => Err("invalid direction"),
            }?)
        .rem_euclid(100);
        if number == 0 {
            count += 1;
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "3");
    }
}
