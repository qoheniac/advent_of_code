//! # Day 2: Rock Paper Scissors
//!
//! [puzzle site](https://adventofcode.com/2022/day/2)

use std::collections::HashMap;

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let action_index =
        HashMap::<_, i32>::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
    let mut score = 0;
    for round in input.lines() {
        let mut actions = round.split_whitespace();
        let action_index_1 = action_index.get(actions.next().unwrap()).unwrap();
        let action_index_2 = action_index.get(actions.next().unwrap()).unwrap();
        score += action_index_2 + 1; // shape score
        score += match (action_index_2 - action_index_1).rem_euclid(3) {
            0 => 3, // draw
            1 => 6, // win
            _ => 0, // loose
        };
    }
    Ok(score.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let action_index = HashMap::<_, i32>::from([("A", 0), ("B", 1), ("C", 2)]);
    let result_index = HashMap::<_, i32>::from([("X", 2), ("Y", 0), ("Z", 1)]);
    let mut score = 0;
    for round in input.lines() {
        let mut actions = round.split_whitespace();
        let action_index_1 = action_index.get(actions.next().unwrap()).unwrap();
        let action_index_difference = result_index.get(actions.next().unwrap()).unwrap();
        let action_index_2 = (action_index_1 + action_index_difference).rem_euclid(3);
        score += action_index_2 + 1; // shape score
        score += match action_index_difference {
            0 => 3, // draw
            1 => 6, // win
            _ => 0, // loose
        };
    }
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "12");
    }
}
