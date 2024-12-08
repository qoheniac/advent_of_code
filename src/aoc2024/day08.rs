//! # Day 8: Resonant Collinearity
//!
//! The input is a map of antenna locations marked by characters other than '.'
//! and each pair of antennas with the same character produces antinodes on the
//! line going through both antennas withing the boundaries of the map. The task
//! is to count all positions with antinodes.
//!
//! [puzzle site](https://adventofcode.com/2024/day08)

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Map {
    width: u8,
    height: u8,
    antennas: HashMap<char, Vec<[u8; 2]>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or("empty input")?.chars().count() as u8;
        let height = s.lines().count() as u8;
        let mut antennas = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character == '.' {
                    continue;
                }
                let location = [i as u8, j as u8];
                antennas
                    .entry(character)
                    .and_modify(|locations: &mut Vec<_>| locations.push(location))
                    .or_insert(vec![location]);
            }
        }
        Ok(Self {
            width,
            height,
            antennas,
        })
    }
}

/// Part 1: Antinodes are only found exactly opposite of the partner antenna
pub fn part1(input: String) -> crate::PuzzleResult {
    let map: Map = input.parse()?;
    let mut antinodes = HashSet::new();
    for locations in map.antennas.values() {
        for location_1 in locations {
            for location_2 in locations {
                if location_1 != location_2 {
                    if let [Some(i), Some(j)] =
                        [0, 1].map(|k| (2 * location_1[k]).checked_sub(location_2[k]))
                    {
                        if i < map.height && j < map.width {
                            antinodes.insert([i, j]);
                        }
                    }
                }
            }
        }
    }
    Ok(antinodes.len().to_string())
}

fn cancel([mut a, mut b]: [i8; 2]) -> [i8; 2] {
    for factor in 2.. {
        if factor > a.abs() || factor > b.abs() {
            break;
        }
        while a % factor == 0 && b % factor == 0 {
            a /= factor;
            b /= factor;
        }
    }
    [a, b]
}

/// Part 2: Antinodes are found everywhere on the line through both antennas
pub fn part2(input: String) -> crate::PuzzleResult {
    let map: Map = input.parse()?;
    let mut antinodes = HashSet::new();
    for locations in map.antennas.values() {
        for (n, location_1) in locations.iter().enumerate() {
            for location_2 in &locations[(n + 1)..] {
                let dr = cancel([0, 1].map(|k| location_2[k] as i8 - location_1[k] as i8));
                let r0 = location_1.map(|m| m as i8);
                antinodes.insert(*location_1);
                for op in [|a, b| a + b, |a, b| a - b] {
                    for steps in 1.. {
                        let [i, j] = [0, 1].map(|k| op(r0[k], steps * dr[k]));
                        if (0..map.height as i8).contains(&i) && (0..map.width as i8).contains(&j) {
                            antinodes.insert([i as u8, j as u8]);
                            continue;
                        }
                        break;
                    }
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

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "34");
    }
}
