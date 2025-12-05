//! # Day 4: Printing Department
//!
//! The input is a map of empty spaces (.) and paper rolls (@). A roll is
//! accessible if its neighbored by at most three paper rolls (horizontally,
//! vertically, or diagonally). Accessible rolls can be removed.
//!
//! [puzzle site](https://adventofcode.com/2025/day/4)

fn get_accessible_rolls(map: &[Vec<char>]) -> Vec<[usize; 2]> {
    let height = map.len();
    let width = map[0].len();
    let mut accessible_rolls = Vec::new();
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
                    accessible_rolls.push([i, j]);
                }
            }
        }
    }
    accessible_rolls
}

/// Part 1: Number of accessible paper rolls
pub fn part1(input: String) -> crate::PuzzleResult {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Ok(get_accessible_rolls(&map).len().to_string())
}

/// Part 2: Number of removable paper rolls
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut roll_count = 0;
    while let accessible_rolls = get_accessible_rolls(&map)
        && !accessible_rolls.is_empty()
    {
        roll_count += accessible_rolls.len();
        for [i, j] in accessible_rolls {
            map[i][j] = '.';
        }
    }
    Ok(roll_count.to_string())
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

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "43");
    }
}
