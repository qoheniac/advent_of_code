//! # Day 2: Gift Shop
//!
//! Comma-separated ranges of IDs need to be searched for invalid ones. IDs are
//! invalid if they consist of a sequence of digits repeated twice.
//!
//! [puzzle site](https://adventofcode.com/2025/day/2)

/// Part 1: Sum of invalid IDs
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for range in input.trim().split(',') {
        let (start, stop) = range.split_once('-').ok_or("invalid string")?;

        // parse start ID
        let half_start_len = start.len() / 2;
        let start_pattern = if start.len() % 2 == 1 {
            10u64.pow(half_start_len as u32)
        } else {
            start[..half_start_len].parse()?
        };
        let start: u64 = start.parse()?;

        // parse stop ID
        let half_stop_len = stop.len() / 2;
        let stop_pattern = if stop.len() % 2 == 1 {
            10u64.pow(half_stop_len as u32) - 1
        } else {
            stop[..half_stop_len].parse()?
        };
        let stop: u64 = stop.parse()?;

        // sum invalid IDs
        for pattern in start_pattern..=stop_pattern {
            let id = pattern * 10u64.pow(pattern.ilog10() + 1) + pattern;
            if (start..=stop).contains(&id) {
                sum += id;
            }
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,",
        "38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "1227775554");
    }
}
