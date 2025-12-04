//! # Day 4: Printing Department
//!
//! The input is a map of empty spaces (.) and paper rolls (@). The solution is
//! the number of accessible paper rolls, that is rolls that are neighbored by
//! at most three paper rolls (horizontally, vertically, or diagonally).
//!
//! [puzzle site](https://adventofcode.com/2025/day/2)

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == '@' {
                let mut neighbor_count = 0;
                if i > 0 && j > 0 && map[i - 1][j - 1] == '@' {
                    neighbor_count += 1;
                }
                if i > 0 && map[i - 1][j] == '@' {
                    neighbor_count += 1;
                }
                if i > 0 && j + 1 < width && map[i - 1][j + 1] == '@' {
                    neighbor_count += 1;
                }
                if j > 0 && map[i][j - 1] == '@' {
                    neighbor_count += 1;
                }
                if j + 1 < width && map[i][j + 1] == '@' {
                    neighbor_count += 1;
                }
                if i + 1 < height && j > 0 && map[i + 1][j - 1] == '@' {
                    neighbor_count += 1;
                }
                if i + 1 < height && map[i + 1][j] == '@' {
                    neighbor_count += 1;
                }
                if i + 1 < height && j + 1 < width && map[i + 1][j + 1] == '@' {
                    neighbor_count += 1;
                }
                if neighbor_count < 4 {
                    count += 1
                }
            }
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "..@@.@@@@.\n",
        "@@@.@.@.@@\n",
        "@@@@@.@.@@\n",
        "@.@@@@..@.\n",
        "@@.@@@@.@@\n",
        ".@@@@@@@.@\n",
        ".@.@.@.@@@\n",
        "@.@@@.@@@@\n",
        ".@@@@@@@@.\n",
        "@.@.@@@.@.\n"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "13");
    }
}
