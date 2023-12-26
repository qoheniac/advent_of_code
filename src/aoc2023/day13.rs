//! # Day 13: Point of Incidence
//!
//! The input contains a number matrices separated by empty lines. The matrices
//! consist of two different characters. The goal is to find lines of reflection
//! without or after fixing one smudge (error in the reflection).
//!
//! [puzzle site](https://adventofcode.com/2023/day/13)

fn solution(input: String, with_smudges: bool) -> crate::PuzzleResult {
    let mut sum = 0;
    'pattern: for pattern in input.split("\n\n") {
        let pattern: Vec<Vec<char>> = pattern.lines().map(|line| line.chars().collect()).collect();
        let width = pattern[0].len();
        let height = pattern.len();

        // search horizontal reflection
        'reflection: for row in 1..height {
            let mut smudge_fixed = !with_smudges;
            for delta in 0..row.min(height - row) {
                for column in 0..width {
                    if pattern[row - delta - 1][column] != pattern[row + delta][column] {
                        if smudge_fixed {
                            continue 'reflection;
                        }
                        smudge_fixed = with_smudges;
                    }
                }
            }
            if smudge_fixed {
                sum += row * 100;
                continue 'pattern;
            }
        }

        // search vertical reflection
        'reflection: for column in 1..width {
            let mut smudge_fixed = !with_smudges;
            for delta in 0..column.min(width - column) {
                for row in 0..height {
                    if pattern[row][column - delta - 1] != pattern[row][column + delta] {
                        if smudge_fixed {
                            continue 'reflection;
                        }
                        smudge_fixed = with_smudges;
                    }
                }
            }
            if smudge_fixed {
                sum += column;
                continue 'pattern;
            }
        }
    }
    Ok(sum.to_string())
}

/// Part 1: Without smudges
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, false)
}

/// Part 2: With smudges
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, true)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
        "\n",
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "405");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "400");
    }
}
