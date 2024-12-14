//! # Day 14: Restroom Redoubt
//!
//! The input has lines of the form "p=-?\d,-?\d v=-?\d,-?\d", where p is the
//! position of a robot and v its velocity. They move in a space that is 101
//! tiles wide and 103 tiles high and teleport to the opposite edge whenever
//! they're about to leave this area.
//!
//! [puzzle site](https://adventofcode.com/2024/day14)

fn parse_vector(string: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let (j, i) = string
        .split_once("=")
        .and_then(|(_, numbers)| numbers.split_once(","))
        .ok_or(format!("invalid vector: {string}"))?;
    Ok((i.parse()?, j.parse()?))
}

struct Robots {
    number: usize,
    positions: Vec<(i64, i64)>,
    velocities: Vec<(i64, i64)>,
}

impl Robots {
    fn update(&mut self, width: i64, height: i64) {
        self.update_with_time(width, height, 1);
    }

    fn update_with_time(&mut self, width: i64, height: i64, time: i64) {
        for i in 0..self.number {
            self.positions[i].0 =
                (self.positions[i].0 + self.velocities[i].0 * time).rem_euclid(height);
            self.positions[i].1 =
                (self.positions[i].1 + self.velocities[i].1 * time).rem_euclid(width);
        }
    }

    fn position_mean(&self) -> (f64, f64) {
        (
            self.positions.iter().map(|p| p.0 as f64).sum::<f64>() / self.number as f64,
            self.positions.iter().map(|p| p.1 as f64).sum::<f64>() / self.number as f64,
        )
    }

    fn position_variance(&self) -> (f64, f64) {
        let (i0, j0) = self.position_mean();
        (
            self.positions
                .iter()
                .map(|p| (p.0 as f64 - i0).powi(2))
                .sum::<f64>()
                / self.number as f64,
            self.positions
                .iter()
                .map(|p| (p.1 as f64 - j0).powi(2))
                .sum::<f64>()
                / self.number as f64,
        )
    }
}

impl std::str::FromStr for Robots {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number = 0;
        let mut positions = Vec::new();
        let mut velocities = Vec::new();
        for line in s.lines() {
            let (position, velocity) =
                (line.split_once(" ")).ok_or(format!("invalid line: {line}"))?;
            positions.push(parse_vector(position)?);
            velocities.push(parse_vector(velocity)?);
            number += 1;
        }
        Ok(Self {
            number,
            positions,
            velocities,
        })
    }
}

impl std::fmt::Display for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i_max = self.positions.iter().map(|p| p.0).max().unwrap_or_default();
        let j_max = self.positions.iter().map(|p| p.1).max().unwrap_or_default();
        for i in 0..=i_max {
            for j in 0..=j_max {
                f.write_str(
                    &match self.positions.iter().filter(|p| **p == (i, j)).count() {
                        0 => " ".to_owned(),
                        n => n.to_string(),
                    },
                )?;
            }
            if i < i_max {
                use std::fmt::Write;
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn part1_with_dimensions(input: String, width: i64, height: i64) -> crate::PuzzleResult {
    let mut robots: Robots = input.parse()?;
    for _ in 0..100 {
        robots.update(width, height);
    }
    let mut counts = [0; 4];
    for position in robots.positions {
        counts[match position {
            (i, j) if i < height / 2 && j > width / 2 => 0,
            (i, j) if i < height / 2 && j < width / 2 => 1,
            (i, j) if i > height / 2 && j < width / 2 => 2,
            (i, j) if i > height / 2 && j > width / 2 => 3,
            _ => continue,
        }] += 1;
    }
    Ok(counts.into_iter().product::<usize>().to_string())
}

/// Part 1: Product of robot numbers in each quadrant after 100 iterations
pub fn part1(input: String) -> crate::PuzzleResult {
    part1_with_dimensions(input, 101, 103)
}

/// Part 2: Number of iterations until the robots display a tree
pub fn part2(input: String) -> crate::PuzzleResult {
    let (width, height) = (101i64, 103i64);
    let mut robots: Robots = input.parse()?;
    let mut min_i_var = height.pow(2) as f64;
    let mut min_i_var_time = 0;
    let mut min_j_var = width.pow(2) as f64;
    let mut min_j_var_time = 0;
    for time in 0..width.max(height) {
        let (i_var, j_var) = robots.position_variance();
        if i_var < min_i_var && time < height {
            min_i_var = i_var;
            min_i_var_time = time;
        }
        if j_var < min_j_var && time < width {
            min_j_var = j_var;
            min_j_var_time = time;
        }
        robots.update(width, height);
    }
    for leap in 0.. {
        let offset = width * leap + min_j_var_time - min_i_var_time;
        if offset % height == 0 {
            // robots.update_with_time(width, height, min_i_var_time + offset - width.max(height));
            // println!("{robots}");
            return Ok((min_i_var_time + offset).to_string());
        }
    }
    Err("no solution found")?
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "p=0,4 v=3,-3\n",
        "p=6,3 v=-1,-3\n",
        "p=10,3 v=-1,2\n",
        "p=2,0 v=2,-1\n",
        "p=0,0 v=1,3\n",
        "p=3,0 v=-2,-2\n",
        "p=7,6 v=-1,-3\n",
        "p=3,0 v=-1,-2\n",
        "p=9,3 v=2,3\n",
        "p=7,3 v=-1,2\n",
        "p=2,4 v=2,-3\n",
        "p=9,5 v=-3,-3",
    );

    #[test]
    fn test_part1() {
        assert_eq!(
            &super::part1_with_dimensions(INPUT.to_string(), 11, 7).unwrap(),
            "12"
        );
    }
}
