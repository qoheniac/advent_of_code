//! # Day 1: Trebuchet?!
//!
//! First and last digit of each line form the calibration value. Find the sum
//! of all calibration values.

/// Part 1: Digits are single numerical characters
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

/// Part 2: Digits might be spelled out
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut digits = std::collections::HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
    for digit in 0..10 {
        digits.insert(digit.to_string(), digit);
    }
    let mut sum: usize = 0;
    for line in input.lines() {
        let first = digits[digits
            .keys()
            .filter_map(|key| line.find(key).and_then(|index| Some((key, index))))
            .min_by(|a, b| a.1.cmp(&b.1))
            .ok_or(format!("no digit in {line}"))?
            .0];
        let last = digits[digits
            .keys()
            .filter_map(|key| line.rfind(key).and_then(|index| Some((key, index))))
            .max_by(|a, b| a.1.cmp(&b.1))
            .ok_or(format!("no two digits in {line}"))?
            .0];
        let number: usize = format!("{first}{last}").parse()?;
        sum += number;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet"
            .to_string();
        assert_eq!(&super::part1(input).unwrap(), "142");
    }

    #[test]
    fn test_part2() {
        let input = "\
            two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen"
            .to_string();
        assert_eq!(&super::part2(input).unwrap(), "281");
    }
}
