//! # Day 9: Movie Theater
//!
//! The input holds tile locations on grid. Each line has one location written
//! as row index comma column index and adjacent tiles as well as the first and
//! last one are always on the same row or column forming a loop through space.
//!
//! [puzzle site](https://adventofcode.com/2025/day/9)

use std::error::Error;

fn edges_and_areas(input: String) -> Result<(Vec<[u64; 4]>, Vec<([u64; 4], u64)>), Box<dyn Error>> {
    let mut corners: Vec<[u64; 2]> = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(",").ok_or("invalid line")?;
        corners.push([x.parse()?, y.parse()?])
    }
    let mut edges = Vec::new();
    let mut areas = Vec::new();
    for (i, &[x0, y0]) in corners.iter().enumerate() {
        let [x1, y1] = corners[(i + 1) % corners.len()];
        edges.push([x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)]);
        for &[x1, y1] in corners.iter().skip(i + 1) {
            areas.push((
                [x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)],
                (1 + x0.abs_diff(x1)) * (1 + y0.abs_diff(y1)),
            ));
        }
    }
    areas.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    Ok((edges, areas))
}

/// Part 1: Largest area of any rectangle spanned by two input tiles
pub fn part1(input: String) -> crate::PuzzleResult {
    Ok(edges_and_areas(input)?.1[0].1.to_string())
}

/// Part 2: Largest area of any rectangle spanned by two input tiles that lies
/// completely within the loop
pub fn part2(input: String) -> crate::PuzzleResult {
    let (edges, areas) = edges_and_areas(input)?;
    'areas: for ([x0, y0, x1, y1], area) in areas {
        for &[x2, y2, x3, y3] in &edges {
            if x0 < x3 && x2 < x1 && y0 < y3 && y2 < y1 {
                continue 'areas;
            }
        }
        return Ok(area.to_string());
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "50");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "24");
    }
}
