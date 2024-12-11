//! # Day 10: Hoof It
//!
//! The input contains a topographical map with heights between 0 and 9. Hiking
//! trails start at 0 and increase one in height each step until reaching 9.
//!
//! [puzzle site](https://adventofcode.com/2024/day10)

use std::{collections::VecDeque, str::FromStr};

struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<u32>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or("empty input")?.chars().count();
        let height = s.lines().count();
        let data = (s.lines())
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        Ok(Self {
            width,
            height,
            data,
        })
    }
}

impl Map {
    fn neighbors(&self, [i, j]: [usize; 2]) -> Vec<[usize; 2]> {
        let mut neighbors = Vec::new();
        if j + 1 < self.width {
            neighbors.push([i, j + 1]);
        }
        if i > 0 {
            neighbors.push([i - 1, j]);
        }
        if j > 0 {
            neighbors.push([i, j - 1]);
        }
        if i + 1 < self.height {
            neighbors.push([i + 1, j]);
        }
        neighbors
    }

    fn get(&self, [i, j]: [usize; 2]) -> u32 {
        self.data[i][j]
    }
}

/// Part 1: Sum up how many 9s can be reached from each 0 over all 0s
pub fn part1(input: String) -> crate::PuzzleResult {
    let map: Map = input.parse()?;
    let mut trailheads = Vec::new();
    for i in 0..map.height {
        for j in 0..map.width {
            if map.data[i][j] == 0 {
                trailheads.push([i, j]);
            }
        }
    }
    let mut sum = 0;
    for [i, j] in trailheads {
        let mut locations = VecDeque::from([[i, j]]);
        for target in 1..=9 {
            for _ in 0..locations.len() {
                for neighbor in map.neighbors(locations.pop_front().unwrap()) {
                    if map.get(neighbor) == target {
                        if !locations.contains(&neighbor) {
                            locations.push_back(neighbor);
                        }
                    }
                }
            }
        }
        sum += locations.len();
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "89010123\n",
        "78121874\n",
        "87430965\n",
        "96549874\n",
        "45678903\n",
        "32019012\n",
        "01329801\n",
        "10456732",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "36");
    }
}
