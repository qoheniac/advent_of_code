//! # Day 12: Hot Springs
//!
//! Each input line holds spring status records and a list with the lengths of
//! all contiguous groups of damaged (#) springs. Some statuses are unknown (?).
//! The goal is to find the sum of the number of possible arrangements for each
//! line. The input might be extended by repeating status records separating
//! them with question marks and also repeating the group length accordingly
//! separating them with commas.
//!
//! [puzzle site](https://adventofcode.com/2023/day/12)

#[derive(Clone, PartialEq)]
enum Status {
    Damaged,
    Operational,
    Unknown,
}
use Status::*;

impl Status {
    fn try_from_char(s: char) -> Result<Self, String> {
        Ok(match s {
            '#' => Damaged,
            '.' => Operational,
            '?' => Unknown,
            _ => Err(format!("{s} is not a valid status"))?,
        })
    }
}

fn arrangements(statuses: Vec<Status>, lengths: &[usize]) -> usize {
    if let Some((&length, next_lengths)) = lengths.split_first() {
        let total_length = statuses.len();
        if length > total_length {
            return 0;
        }
        let mut sum = 0;
        for i in 0..(total_length + 1 - length) {
            let after_group = i + length;
            // No match
            if statuses[i..after_group].contains(&Operational) {
                if statuses[i] == Damaged {
                    // Should have been a match
                    return sum; // No further arrangements possible
                } else {
                    continue; // Moving on
                }
            }
            // Matching end
            if after_group == total_length {
                if next_lengths.is_empty() {
                    // Last Group
                    sum += 1; // Arrangement found
                }
                break;
            }
            // No match because group would be too long
            if statuses[after_group] == Damaged {
                if statuses[i] == Damaged {
                    // Should have been a match
                    return sum; // No further arrangements possible
                } else {
                    continue; // Moving on
                }
            }
            // Match
            let mut next_statuses = statuses[after_group..].to_vec();
            next_statuses[0] = Operational;
            sum += arrangements(next_statuses, next_lengths);
            if statuses[i] == Damaged {
                return sum; // No further arrangements possible
            }
        }
        sum
    } else {
        !statuses.contains(&Damaged) as usize
    }
}

fn solution(input: String, repeat: usize) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let (statuses, lengths) = line.split_once(" ").ok_or("missing space")?;
        let statuses = format!("?{statuses}").repeat(repeat)[1..]
            .chars()
            .map(Status::try_from_char)
            .collect::<Result<Vec<_>, _>>()?;
        let lengths = format!(",{lengths}").repeat(repeat)[1..]
            .split(",")
            .map(|number| number.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        sum += arrangements(statuses, &lengths);
    }
    Ok(sum.to_string())
}

/// Part 1: Don't fold
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 1)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "???.### 1,1,3\n",
        ".??..??...?##. 1,1,3\n",
        "?#?#?#?#?#?#?#? 1,3,1,6\n",
        "????.#...#... 4,1,1\n",
        "????.######..#####. 1,6,5\n",
        "?###???????? 3,2,1"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "21");
    }
}
