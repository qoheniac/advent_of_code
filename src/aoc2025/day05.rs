//! # Day 5: Cafeteria
//!
//! The input has two blocks separated by a blank line. The first block is lines
//! of inclusive ranges of fresh ingredient IDs with a start-end format and the
//! second block holds one available ingredient ID per line.
//!
//! [puzzle site](https://adventofcode.com/2025/day/5)

fn parse_ranges(
    range_block: &str,
) -> Result<Vec<std::ops::RangeInclusive<u64>>, Box<dyn std::error::Error>> {
    let mut ranges = Vec::new();
    for range in range_block.lines() {
        let (start, end) = range.split_once('-').ok_or("invalid range")?;
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        ranges.push(start..=end);
    }
    Ok(ranges)
}

/// Part 1: Number of available fresh ingredients
pub fn part1(input: String) -> crate::PuzzleResult {
    let (range_block, id_block) = input.split_once("\n\n").ok_or("no blank line")?;
    let ranges = parse_ranges(range_block)?;
    let mut fresh_ids_count = 0;
    for id in id_block.lines() {
        let id: u64 = id.parse()?;
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids_count += 1;
                break;
            }
        }
    }
    Ok(fresh_ids_count.to_string())
}

/// Part 2: Total number of fresh ingredient IDs
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut ranges = parse_ranges(input.split_once("\n\n").ok_or("no blank line")?.0)?;
    let mut i = 0;
    loop {
        if i == ranges.len() {
            break;
        }
        let mut j = 0;
        loop {
            if j == ranges.len() {
                break;
            }
            if i != j
                && ranges[i].start() <= ranges[j].end()
                && ranges[j].start() <= ranges[i].end()
            {
                ranges[i] = (*ranges[i].start()).min(*ranges[j].start())
                    ..=(*ranges[i].end()).max(*ranges[j].end());
                ranges.remove(j);
                if j < i {
                    i -= 1;
                }
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    Ok((ranges.iter().map(|r| 1 + r.end() - r.start()).sum::<u64>()).to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "14");
    }
}
