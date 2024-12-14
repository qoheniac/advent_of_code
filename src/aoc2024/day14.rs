//! # Day 14: Restroom Redoubt
//!
//! The input has lines of the form "p=-?\d,-?\d v=-?\d,-?\d", where p is the
//! position of a robot and v its velocity. They move in a space that is 101
//! tiles wide and 103 tiles high and teleport to the opposite edge whenever
//! they're about to leave this area.
//!
//! [puzzle site](https://adventofcode.com/2024/day14)

fn parse_vector(string: &str) -> Result<(i16, i16), Box<dyn std::error::Error>> {
    let (i, j) = string
        .split_once("=")
        .and_then(|(_, numbers)| numbers.split_once(","))
        .ok_or(format!("invalid vector: {string}"))?;
    Ok((i.parse()?, j.parse()?))
}

fn solution(input: String, width: i16, height: i16) -> crate::PuzzleResult {
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for line in input.lines() {
        let (position, velocity) = (line.split_once(" ")).ok_or(format!("invalid line: {line}"))?;
        positions.push(parse_vector(position)?);
        velocities.push(parse_vector(velocity)?);
    }
    for _ in 0..100 {
        for i in 0..positions.len() {
            positions[i].0 = (positions[i].0 + velocities[i].0).rem_euclid(width);
            positions[i].1 = (positions[i].1 + velocities[i].1).rem_euclid(height);
        }
    }
    let mut counts = [0; 4];
    for position in positions {
        counts[match position {
            (i, j) if i < width / 2 && j > height / 2 => 0,
            (i, j) if i < width / 2 && j < height / 2 => 1,
            (i, j) if i > width / 2 && j < height / 2 => 2,
            (i, j) if i > width / 2 && j > height / 2 => 3,
            _ => continue,
        }] += 1;
    }
    Ok(counts.into_iter().product::<usize>().to_string())
}

/// Part 1: Product of robot numbers in each quadrant after 100 iterations
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 101, 103)
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
        assert_eq!(&super::solution(INPUT.to_string(), 11, 7).unwrap(), "12");
    }
}
