//! # Day 5: Cafeteria
//!
//! The input has two blocks separated by a blank line. The first block is lines
//! of inclusive ranges of fresh ingredient IDs with a start-end format and the
//! second block holds one available ingredient ID per line.
//!
//! [puzzle site](https://adventofcode.com/2025/day/5)

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let (range_block, id_block) = input.split_once("\n\n").ok_or("no blank line")?;

    let mut ranges = Vec::new();
    for range in range_block.lines() {
        let (start, end) = range.split_once('-').ok_or("invalid range")?;
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        ranges.push(start..=end);
    }

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

#[cfg(test)]
mod tests {
    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "3");
    }
}
