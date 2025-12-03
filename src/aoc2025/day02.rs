//! # Day 2: Gift Shop
//!
//! The input is one line of comma-separated ranges of IDs in the format
//! start-end. The solution is the sum of all invalid IDs in all ranges.
//!
//! [puzzle site](https://adventofcode.com/2025/day/2)

fn parse_input(input: String) -> Result<Vec<[u64; 2]>, Box<dyn std::error::Error>> {
    let mut pairs = Vec::new();
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').ok_or("invalid string")?;
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;
        pairs.push([start, end]);
    }
    Ok(pairs)
}

fn invalid_factor(sequence_len: u32, repetitions: u32) -> u64 {
    (10u64.pow(repetitions * sequence_len) - 1) / (10u64.pow(sequence_len) - 1)
}

fn sum_ids_with_repetitions(start: u64, end: u64, repetitions: u32, only_exact: bool) -> u64 {
    let start_len = start.ilog10() + 1;
    let start_sequence_len = start_len.div_ceil(repetitions);
    let start_sequence = if start_len == start_sequence_len * repetitions {
        start / 10u64.pow(start_len - start_sequence_len)
    } else {
        10u64.pow(start_sequence_len - 1)
    };
    let end_len = end.ilog10() + 1;
    let end_sequence_len = end_len / repetitions;
    let end_sequence = if end_len == end_sequence_len * repetitions {
        end / 10u64.pow(end_len - end_sequence_len)
    } else {
        10u64.pow(end_sequence_len) - 1
    };
    let mut sum = 0;
    'sequences: for sequence in start_sequence..=end_sequence {
        let sequence_len = sequence.ilog10() + 1;
        if only_exact {
            for subsequence_len in 1..=(sequence_len / 2) {
                if sequence_len.is_multiple_of(subsequence_len) {
                    let sub_repetitions = sequence_len / subsequence_len;
                    if sequence.is_multiple_of(invalid_factor(subsequence_len, sub_repetitions)) {
                        continue 'sequences;
                    }
                }
            }
        }
        let id = sequence * invalid_factor(sequence_len, repetitions);
        if (start..=end).contains(&id) {
            sum += id;
        }
    }
    sum
}

/// Part 1: Invalid IDs consist of a sequence of digits repeated twice.
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for [start, end] in parse_input(input)? {
        sum += sum_ids_with_repetitions(start, end, 2, false);
    }
    Ok(sum.to_string())
}

/// Part 2: Invalid IDs consist of a sequence of digits repeated at least twice.
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for [start, end] in parse_input(input)? {
        for n in 2..=(end.ilog10() + 1) {
            sum += sum_ids_with_repetitions(start, end, n, true);
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

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "4174379265");
    }
}
