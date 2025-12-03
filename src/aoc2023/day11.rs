//! # Day 11: Cosmic Expansion
//!
//! The input holds a map of galaxies (#) and the goal is to find the sum of all
//! mutual taxicab distances.
//!
//! [puzzle site](https://adventofcode.com/2023/day/11)

use itertools::Itertools;

fn sort_pair((a, b): (usize, usize)) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}

fn count_between(vector: &[bool], start: usize, end: usize) -> usize {
    vector[start..end].iter().filter(|&&b| b).count()
}

fn solution(input: String, age: usize) -> crate::PuzzleResult {
    let mut rows = 0;
    let mut cols = 0;
    let mut galaxies = Vec::new();
    for line in input.lines() {
        cols = 0;
        for character in line.chars() {
            if character == '#' {
                galaxies.push((rows, cols));
            }
            cols += 1;
        }
        rows += 1;
    }
    let mut is_row_empty = vec![true; rows];
    let mut is_col_empty = vec![true; cols];
    for &(row, col) in &galaxies {
        is_row_empty[row] = false;
        is_col_empty[col] = false;
    }
    let mut sum = 0;
    for (&(row1, col1), &(row2, col2)) in galaxies.iter().tuple_combinations() {
        let (top, bottom) = sort_pair((row1, row2));
        let (left, right) = sort_pair((col1, col2));
        sum += bottom - top + right - left
            + count_between(&is_row_empty, top, bottom) * (age - 1)
            + count_between(&is_col_empty, left, right) * (age - 1);
    }
    Ok(sum.to_string())
}

/// Part 1: Empty rows and columns expand by two
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 2)
}

/// Part 2: Empty rows and columns expand by a million
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, 1000000)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "...#......\n",
        ".......#..\n",
        "#.........\n",
        "..........\n",
        "......#...\n",
        ".#........\n",
        ".........#\n",
        "..........\n",
        ".......#..\n",
        "#...#....."
    );

    fn test(age: usize, result: &str) {
        assert_eq!(&super::solution(INPUT.to_string(), age).unwrap(), result);
    }

    #[test]
    fn test_part1() {
        test(2, "374");
    }

    #[test]
    fn test_part2() {
        test(10, "1030");
        test(100, "8410");
    }
}
