//! # Day 8: Resonant Collinearity
//!
//! The input is a map of antenna locations marked by characters other than '.'
//! and each pair of antennas with the same character produces antinodes on both
//! sides of the pair in a distance equal to that between the antennas.
//!
//! [puzzle site](https://adventofcode.com/2024/day08)

use std::collections::{HashMap, HashSet};

/// Part 1: Number of unique antinodes within the maps boundaries
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut antennas = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }
            let location = [i as i8, j as i8];
            antennas
                .entry(character)
                .and_modify(|locations: &mut Vec<_>| locations.push(location))
                .or_insert(vec![location]);
        }
    }
    let height = input.lines().count() as i8;
    let width = input.lines().next().ok_or("empty input")?.chars().count() as i8;
    let mut antinodes = HashSet::new();
    for locations in antennas.values() {
        for location_1 in locations {
            for location_2 in locations {
                if location_1 == location_2 {
                    continue;
                }
                let antinode = [0, 1].map(|k| 2 * location_1[k] - location_2[k]);
                if (0..height).contains(&antinode[0]) && (0..width).contains(&antinode[1]) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "............\n",
        "........0...\n",
        ".....0......\n",
        ".......0....\n",
        "....0.......\n",
        "......A.....\n",
        "............\n",
        "............\n",
        "........A...\n",
        ".........A..\n",
        "............\n",
        "............"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "14");
    }
}
