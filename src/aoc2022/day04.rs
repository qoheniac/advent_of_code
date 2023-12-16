//! # Day 4: Camp Cleanup
//!
//! [puzzle site](https://adventofcode.com/2022/day/4)

fn parse_line(line: &str) -> Result<(i32, i32, i32, i32), Box<dyn std::error::Error>> {
    let (range1, range2) = line.split_once(",").ok_or("no ranges found")?;
    let (lower1, upper1) = range1.split_once("-").ok_or("cannot parse range 1")?;
    let (lower2, upper2) = range2.split_once("-").ok_or("cannot parse range 2")?;
    Ok((
        lower1.parse()?,
        upper1.parse()?,
        lower2.parse()?,
        upper2.parse()?,
    ))
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut count = 0;
    for line in input.lines() {
        let (lower1, upper1, lower2, upper2) = parse_line(line)?;
        if (lower1 - lower2) * (upper2 - upper1) >= 0 {
            count += 1;
        }
    }
    Ok(count.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut count = 0;
    for line in input.lines() {
        let (lower1, upper1, lower2, upper2) = parse_line(line)?;
        if upper2 >= lower1 && lower2 <= upper1 {
            count += 1;
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
        2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "4");
    }
}
