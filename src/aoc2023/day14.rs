//! # Day 14: Parabolic Reflector Dish
//!
//! The input holds a rectangular map of round stones (O), cubic stones (#) and
//! empty space (.) on a platform. Tilting the platform north, west, south, or
//! east lets all round stones roll as far as possible in the respective
//! direction. The goal is to find the total load, where each round stone adds a
//! load equal to its distance from the southern end.
//!
//! [puzzle site](https://adventofcode.com/2023/day/14)

use ndarray::Array2;

#[derive(Eq, Hash, Clone, Copy, PartialEq)]
enum Field {
    Ball,
    Cube,
    Space,
}
use Field::*;

enum Direction {
    North,
    West,
    South,
    East,
}
use Direction::*;

#[derive(Eq, Hash, Clone, PartialEq)]
struct Platform(Array2<Field>);

impl Platform {
    fn load(&self) -> usize {
        let mut sum = 0;
        for column in self.0.columns() {
            for (row, field) in column.iter().enumerate() {
                if let Ball = field {
                    sum += self.0.nrows() - row;
                }
            }
        }
        sum
    }

    fn tilt(&mut self, direction: Direction) {
        let end = match direction {
            North | South => self.0.nrows(),
            West | East => self.0.ncols(),
        } as i32;
        for mut line in match direction {
            North | South => self.0.columns_mut(),
            West | East => self.0.rows_mut(),
        } {
            let mut cube = match direction {
                North | West => -1,
                South | East => end,
            };
            let mut count = 0;
            for mut position in 0..=end {
                if let South | East = direction {
                    position = end - position - 1;
                }
                match if !(0..end).contains(&position) {
                    Cube
                } else {
                    line[position as usize]
                } {
                    Space => (),
                    Ball => count += 1,
                    Cube => {
                        // Sort balls and spaces between two cubes
                        for index in match direction {
                            North | West => (cube + 1)..position,
                            South | East => (position + 1)..cube,
                        } {
                            line[index as usize] = if cube.abs_diff(index) <= count {
                                Ball
                            } else {
                                Space
                            };
                        }
                        cube = position;
                        count = 0;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt(North);
        self.tilt(West);
        self.tilt(South);
        self.tilt(East);
    }
}

impl std::str::FromStr for Platform {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            height += 1;
            width = 0;
            for character in line.chars() {
                width += 1;
                fields.push(match character {
                    'O' => Ball,
                    '#' => Cube,
                    '.' => Space,
                    c => Err(format!("unexpected character {c}"))?,
                })
            }
        }
        Ok(Platform(Array2::from_shape_vec((height, width), fields)?))
    }
}

/// Part 1: Tilt north
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut platform: Platform = input.parse()?;
    platform.tilt(North);
    Ok(platform.load().to_string())
}

/// Part 2: Tilt north, west, south, and east a billion times
pub fn part2(input: String) -> crate::PuzzleResult {
    const CYCLES: usize = 1000000000;
    let mut platform: Platform = input.parse()?;
    let mut hashes = std::collections::HashMap::new();
    for cycle in 0..CYCLES {
        platform.cycle();
        // Loop detection
        if let Some(index) = hashes.get(&platform) {
            for _ in 0..((CYCLES - cycle - 1) % (cycle - index)) {
                platform.cycle();
            }
            break;
        }
        hashes.insert(platform.clone(), cycle);
    }
    Ok(platform.load().to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "136");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "64");
    }
}
