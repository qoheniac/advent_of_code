//! # Day 18: Lavaduct Lagoon
//!
//! The input contains a dig plan where each line describes the direction and
//! length of a trench segment. The trench forms a loop and the goal is to
//! calculate the volume of the lagoon that forms after digging out the interior
//! of the trench.
//!
//! [puzzle site](https://adventofcode.com/2023/day/18)

use regex::Regex;

enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

// Shoelace formula plus outer half of the trench
fn lagoon_volume(dig_plan: impl Iterator<Item = (Direction, i64)>) -> i64 {
    let mut lagoon_volume = 0;
    let mut trench_length = 0;
    let mut location = (0, 0);
    for (direction, distance) in dig_plan {
        let (x_1, y_1) = location;
        location = match direction {
            Down => (x_1, y_1 - distance),
            Left => (x_1 - distance, y_1),
            Right => (x_1 + distance, y_1),
            Up => (x_1, y_1 + distance),
        };
        let (x_2, y_2) = location;
        lagoon_volume += x_1 * y_2 - x_2 * y_1;
        trench_length += distance;
    }
    (lagoon_volume.abs() + trench_length) / 2 + 1
}

/// Part 1: Direction in first column, distance in second
pub fn part1(input: String) -> crate::PuzzleResult {
    let dig_plan = input.lines().flat_map(|line| {
        let mut split = line.split(' ');
        let direction: Option<Direction> = split.next().and_then(|s| match s {
            "D" => Some(Down),
            "L" => Some(Left),
            "R" => Some(Right),
            "U" => Some(Up),
            _ => None,
        });
        let distance: Option<i64> = split.next().and_then(|s| s.parse().ok());
        direction.zip(distance)
    });
    Ok(lagoon_volume(dig_plan).to_string())
}

/// Part 2: Third column holds direction and distance
///
/// The third column is in parentheses and holds a hex triplet with a leading
/// number sign. The last digit represents the direction and the other digits
/// represent the distance.
pub fn part2(input: String) -> crate::PuzzleResult {
    let re = Regex::new(r"\(#([0-9a-f]{6})\)").unwrap();
    let dig_plan = input.lines().flat_map(|line| {
        re.captures(line).and_then(|cap| {
            cap.get(1).and_then(|m| {
                let mut chars = m.as_str().chars();
                let direction = match chars.next_back() {
                    Some('0') => Some(Right),
                    Some('1') => Some(Down),
                    Some('2') => Some(Left),
                    Some('3') => Some(Up),
                    _ => None,
                };
                let distance = i64::from_str_radix(chars.as_str(), 16).ok();
                direction.zip(distance)
            })
        })
    });
    Ok(lagoon_volume(dig_plan).to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "62");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "952408144115");
    }
}
