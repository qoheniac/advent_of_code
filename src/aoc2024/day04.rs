//! # Day 4: Ceres Search
//!
//! The input holds a rectangular field of letters.
//!
//! [puzzle site](https://adventofcode.com/2024/day04)

#[derive(Clone, Copy)]
enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}
use Direction::*;

struct RectanglePointer {
    height: usize,
    width: usize,
    i: usize,
    j: usize,
}

impl RectanglePointer {
    fn try_step(&mut self, direction: Direction) -> Result<(), ()> {
        (self.i, self.j) = match (self.i, self.j, direction) {
            (0, _, NE | N | NW) | (_, 0, NW | W | SW) => Err(())?,
            (i, _, SW | S | SE) if i == self.height - 1 => Err(())?,
            (_, j, SE | E | NE) if j == self.width - 1 => Err(())?,
            (i, j, _) if i >= self.height || j >= self.width => Err(())?,
            (i, j, E) => (i, j + 1),
            (i, j, NE) => (i - 1, j + 1),
            (i, j, N) => (i - 1, j),
            (i, j, NW) => (i - 1, j - 1),
            (i, j, W) => (i, j - 1),
            (i, j, SW) => (i + 1, j - 1),
            (i, j, S) => (i + 1, j),
            (i, j, SE) => (i + 1, j + 1),
        };
        Ok(())
    }
}

/// Part 1: Count occurences of XMAS in any orientation
pub fn part1(input: String) -> crate::PuzzleResult {
    let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = field.len();
    let width = field[0].len();
    let directions = [E, NE, N, NW, W, SW, S, SE];
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if field[i][j] == 'X' {
                'direction: for direction in directions {
                    let mut pointer = RectanglePointer {
                        width,
                        height,
                        i,
                        j,
                    };
                    for letter in ['M', 'A', 'S'] {
                        if pointer.try_step(direction).is_err()
                            || field[pointer.i][pointer.j] != letter
                        {
                            continue 'direction;
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    Ok(count.to_string())
}

/// Part 2: Count occurences of two MAS forming an X
pub fn part2(input: String) -> crate::PuzzleResult {
    let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = field.len();
    let width = field[0].len();
    let mut count = 0;
    for i in 1..(height - 1) {
        'column: for j in 1..(width - 1) {
            if field[i][j] == 'A' {
                let mut arms = Vec::new();
                for direction in [NE, NW, SW, SE] {
                    let mut pointer = RectanglePointer {
                        width,
                        height,
                        i,
                        j,
                    };
                    pointer.try_step(direction).unwrap();
                    let letter = field[pointer.i][pointer.j];
                    if !['M', 'S'].contains(&letter) {
                        continue 'column;
                    }
                    arms.push(letter);
                }
                if arms[0] != arms[2] && arms[1] != arms[3] {
                    count += 1;
                }
            }
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "MMMSXXMASM\n",
        "MSAMXMSMSA\n",
        "AMXSXMAAMM\n",
        "MSAMASMSMX\n",
        "XMASAMXAMM\n",
        "XXAMMXXAMA\n",
        "SMSMSASXSS\n",
        "SAXAMASAAA\n",
        "MAMMMXMMMM\n",
        "MXMXAXMASX"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "9");
    }
}
