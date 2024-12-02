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

/// Part 1: Number of safe reports
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut number_of_safe_reports = 0;
    for line in input.lines() {
        let mut levels = line.split_whitespace().map(|s| s.parse());
        let mut previous_level = (levels.next()).ok_or(format!("no level in '{line}'"))??;
        let mut level = (levels.next()).ok_or(format!("only one level in '{line}"))??;
        let mut report = check_levels(level, previous_level);
        for next in levels {
            if report == Report::Unsafe {
                break;
            }
            (level, previous_level) = (next?, level);
            let interim_report = check_levels(level, previous_level);
            if interim_report != report {
                report = Report::Unsafe
            }
        }
        if let Report::Safe(_) = report {
            number_of_safe_reports += 1;
        }
    }
    Ok(number_of_safe_reports.to_string())
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
}
