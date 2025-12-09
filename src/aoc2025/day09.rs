//! # Day 9: Movie Theater
//!
//! The input holds tile locations on grid. Each line has one location written
//! as row index comma column index and adjacent tiles as well as the first and
//! last one are always on the same row or column forming a loop through space.
//!
//! [puzzle site](https://adventofcode.com/2025/day/9)

/// Part 1: Largest area of any rectangle spanned by two input tiles
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut positions: Vec<[u64; 2]> = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(",").ok_or("invalid line")?;
        positions.push([x.parse()?, y.parse()?])
    }
    let mut max = 0;
    for (i, [x0, y0]) in positions.iter().enumerate() {
        for &[x1, y1] in positions.iter().skip(i + 1) {
            max = max.max((1 + x0.abs_diff(x1)) * (1 + y0.abs_diff(y1)));
        }
    }
    Ok(max.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "50");
    }
}
