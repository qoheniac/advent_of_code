//! # Day 6: Guard Gallivant
//!
//! The input shows a rectangular grid of free spaces (.) and obstructions (#)
//! with a guard (^) facing upwards. The guard turns right whenever she hits an
//! obstruction and otherwise moves forwards.
//!
//! [puzzle site](https://adventofcode.com/2024/day06)

use std::{collections::HashSet, str::FromStr};

enum Direction {
    Right,
    Up,
    Left,
    Down,
}
use Direction::*;

struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Right => Down,
            Up => Right,
            Left => Up,
            Down => Left,
        }
    }
}

enum Tile {
    Space,
    Obstruction,
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        let mut guard = None;
        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, symbol) in line.chars().enumerate() {
                row.push(match symbol {
                    '^' | '.' => Tile::Space,
                    '#' => Tile::Obstruction,
                    c => Err(format!("invalid character {c}"))?,
                });
                if symbol == '^' {
                    guard = Some(Guard {
                        position: (i, j),
                        direction: Direction::Up,
                    });
                }
            }
            tiles.push(row);
        }
        if !tiles.windows(2).all(|w| w[0].len() == w[1].len()) {
            return Err("lines have different length".to_owned());
        }
        Ok(Self {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles,
            guard: guard.ok_or("no guard found")?,
        })
    }
}

impl Map {
    fn move_guard(&mut self) -> Result<(), ()> {
        let (i, j) = self.guard.position;
        let (i, j) = match self.guard.direction {
            Right if j + 1 < self.width => (i, j + 1),
            Up if i > 0 => (i - 1, j),
            Left if j > 0 => (i, j - 1),
            Down if i + 1 < self.height => (i + 1, j),
            _ => Err(())?,
        };
        match self.tiles[i][j] {
            Tile::Space => self.guard.position = (i, j),
            Tile::Obstruction => {
                self.guard.turn_right();
                self.move_guard()?;
            }
        }
        Ok(())
    }
}

/// Part 1: Count all tiles the guard visited
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut map: Map = input.parse()?;
    let mut seen_positions = HashSet::new();
    seen_positions.insert(map.guard.position);
    while map.move_guard().is_ok() {
        seen_positions.insert(map.guard.position);
    }
    Ok(seen_positions.len().to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#..."
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "41");
    }
}
