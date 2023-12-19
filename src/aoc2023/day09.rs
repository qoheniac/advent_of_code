//! # Day 9: Mirage Maintenance
//!
//! Each line holds a sequence that needs to be continued by successively
//! comparing the differences between neighbors.
//!
//! [puzzle site](https://adventofcode.com/2023/day/9)

enum Direction {
    Left,
    Right,
}
use Direction::*;

fn prediction(sequence: Vec<i64>, direction: &Direction) -> Option<i64> {
    if sequence.iter().all(|d| *d == 0) {
        return Some(0);
    }
    match direction {
        Left => sequence.first(),
        Right => sequence.last(),
    }
    .and_then(|last| {
        prediction(
            sequence.windows(2).map(|w| w[1] - w[0]).collect(),
            direction,
        )
        .and_then(|prediction| {
            Some(match direction {
                Left => last - prediction,
                Right => last + prediction,
            })
        })
    })
}

fn solution(input: String, direction: Direction) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        sum += prediction(
            line.split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<i64>, _>>()?,
            &direction,
        )
        .ok_or(format!("prediction not possible for {line}"))?;
    }
    Ok(sum.to_string())
}

/// Part 1: Sum of first successors
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Right)
}

/// Part 2: Sum of first predecessors
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, Left)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "114");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "2");
    }
}
