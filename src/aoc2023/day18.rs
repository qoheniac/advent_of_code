//! # Day 18: Lavaduct Lagoon
//!
//! [puzzle site](https://adventofcode.com/2023/day/18)

enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

impl Direction {
    fn go(&self, start: &(i32, i32), distance: i32) -> (i32, i32) {
        match self {
            Down => (start.0, start.1 - distance),
            Left => (start.0 - distance, start.1),
            Right => (start.0 + distance, start.1),
            Up => (start.0, start.1 + distance),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            _ => Err(format!("{s} is not a direction")),
        }
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut lagoon_volume = 0;
    let mut trench_length = 0;
    let mut location = (0, 0);
    for line in input.lines() {
        let mut split = line.split(' ');
        let error = format!("cannot parse line {line}");
        let direction: Direction = split.next().ok_or(error.clone())?.parse()?;
        let distance: i32 = split.next().ok_or(error)?.parse()?;
        let (x_1, y_1) = location;
        location = direction.go(&location, distance);
        let (x_2, y_2) = location;
        lagoon_volume += x_1 * y_2 - x_2 * y_1;
        trench_length += distance;
    }
    lagoon_volume = (lagoon_volume.abs() + trench_length) / 2 + 1;
    Ok(lagoon_volume.to_string())
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
}
