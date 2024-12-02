//! # Day 2: Red-Nosed Reports
//!
//! Each line represents a report holding numerical levels which is regarded
//! safe if the levels are strictly ordered with neighbors not differing by more
//! than three.
//!
//! [puzzle site](https://adventofcode.com/2024/day/2)

#[derive(PartialEq)]
enum Order {
    Decreasing,
    Increasing,
}

#[derive(PartialEq)]
enum Report {
    Safe(Order),
    Unsafe,
}

fn check_levels(level: u8, previous_level: u8) -> Report {
    if !(1..=3).contains(&level.abs_diff(previous_level)) {
        Report::Unsafe
    } else if level < previous_level {
        Report::Safe(Order::Decreasing)
    } else {
        Report::Safe(Order::Increasing)
    }
}

impl From<Vec<u8>> for Report {
    fn from(levels: Vec<u8>) -> Self {
        let mut report = check_levels(levels[1], levels[0]);
        let mut previous_level = levels[1];
        for &level in &levels[2..] {
            if report == Report::Unsafe {
                break;
            }
            if check_levels(level, previous_level) != report {
                report = Report::Unsafe
            }
            previous_level = level;
        }
        report
    }
}

fn parse(line: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    line.split_whitespace().map(|s| s.parse()).collect()
}

fn solution(input: String, use_dampener: bool) -> crate::PuzzleResult {
    let mut number_of_safe_reports = 0;
    for line in input.lines() {
        let levels = parse(line)?;
        if Report::from(levels.clone()) != Report::Unsafe {
            number_of_safe_reports += 1;
        } else if use_dampener {
            for i in 0..levels.len() {
                let mut residual = levels.clone();
                residual.remove(i);
                if Report::from(residual) != Report::Unsafe {
                    number_of_safe_reports += 1;
                    break;
                }
            }
        }
    }
    Ok(number_of_safe_reports.to_string())
}

/// Part 1: Number of safe reports
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, false)
}

/// Part 2: Number of safe reports when up to one unsafe level can be ignored
/// per report
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, true)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "7 6 4 2 1\n",
        "1 2 7 8 9\n",
        "9 7 6 2 1\n",
        "1 3 2 4 5\n",
        "8 6 4 4 1\n",
        "1 3 6 7 9"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "4");
    }
}
