//! Day 1: Trebuchet?!
//!
//! First and last digit of each line form the calibration value.

/// Part 1: Sum of all calibration values
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum: usize = 0;
    for line in input.lines() {
        let hits = line.matches(|c: char| c.is_digit(10));
        let first = hits.clone().next().ok_or(format!("no digit in {line}"))?;
        let last = hits.last().ok_or(format!("no two digits in {line}"))?;
        let number: usize = format!("{first}{last}").parse()?;
        sum += number;
    }
    Ok(sum.to_string())
}
