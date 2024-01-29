//! # Day 18: Lavaduct Lagoon
//!
//! [puzzle site](https://adventofcode.com/2023/day/18)

use num::Integer;

enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

impl Direction {
    fn step(&self, start: &(i32, i32)) -> (i32, i32) {
        match self {
            Down => (start.0, start.1 - 1),
            Left => (start.0 - 1, start.1),
            Right => (start.0 + 1, start.1),
            Up => (start.0, start.1 + 1),
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

struct Trench(Vec<(i32, i32)>);

impl Trench {
    fn index_before(&self, index: usize) -> usize {
        index.checked_sub(1).unwrap_or(self.0.len() - 1)
    }

    fn before(&self, index: usize) -> (i32, i32) {
        self.0[self.index_before(index)]
    }

    fn index_after(&self, index: usize) -> usize {
        (index + 1) % self.0.len()
    }

    fn after(&self, index: usize) -> (i32, i32) {
        self.0[self.index_after(index)]
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    // parse trench
    let mut trench = Trench(vec![(0, 0)]);
    for line in input.lines() {
        let mut split = line.split(' ');
        let error = format!("cannot parse line {line}");
        let direction: Direction = split.next().ok_or(error.clone())?.parse()?;
        let distance: usize = split.next().ok_or(error)?.parse()?;
        for _ in 0..distance {
            trench.0.push(direction.step(trench.0.last().unwrap()));
        }
    }
    let top = *trench.0.iter().map(|(_, y)| y).max().ok_or("empty input")?;
    if trench.0.len() < 9 || trench.0.pop().unwrap() != trench.0[0] {
        Err("dig plan did not result in a loop")?;
    }

    // find point inside
    let mut point_inside = None;
    for point in [(1, 1), (1, -1), (-1, -1), (-1, 1)] {
        if trench.0.contains(&point) {
            continue;
        }

        // ray casting
        let mut count = 0;
        let mut location = point;
        while location.1 <= top + 1 {
            location = Up.step(&location);

            // ray hits trench
            if let Some(index) = trench.0.iter().position(|&p| p == location) {
                let mut start_index = index;
                while trench.before(start_index).0 == location.0 {
                    start_index = trench.index_before(start_index);
                    location = Up.step(&location);
                }
                let mut end_index = index;
                while trench.after(end_index).0 == location.0 {
                    end_index = trench.index_after(end_index);
                    location = Up.step(&location);
                }

                // check for intersection (no tangent)
                if trench.before(start_index).0 != trench.after(end_index).0 {
                    count += 1;
                }
            }
        }
        if count.is_odd() {
            point_inside = Some(point);
            break;
        }
    }
    let point_inside = point_inside.ok_or("no point inside trench found")?;

    // flood fill
    let mut interior = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::from([point_inside]);
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        if !trench.0.contains(&point) && interior.insert(point) {
            for direction in [Down, Left, Right, Up] {
                queue.push_back(direction.step(&point));
            }
        }
    }

    // count and return total volume
    Ok((trench.0.len() + interior.len()).to_string())
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
