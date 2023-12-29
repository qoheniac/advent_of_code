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

enum Field {
    Ball,
    Cube,
    Space,
}
use Field::*;

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

    fn tilt_north(&mut self) {
        let rows = self.0.nrows();
        for mut column in self.0.columns_mut() {
            let mut start = 0;
            let mut count = 0;
            for i in 0..rows {
                match column[i] {
                    Space => (),
                    Ball => count += 1,
                    Cube => {
                        for k in start..i {
                            column[k] = if k - start < count { Ball } else { Space };
                        }
                        start = i + 1;
                        count = 0;
                    }
                }
            }
            for k in start..rows {
                column[k] = if k - start < count { Ball } else { Space };
            }
        }
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
    platform.tilt_north();
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
}
