//! # Day 15: Lens Library
//!
//! The input holds a comma-separated list of steps that consist of a label
//! followed by either a dash or an equal sign and a single-digit focal length.
//! A hash function converts strings to integers by starting with zero and for
//! each character add its ASCII code and multiply everything with 17 modulo
//! 256.
//!
//! [puzzle site](https://adventofcode.com/2023/day/15)

use regex::Regex;

fn hash(string: &str) -> usize {
    let mut current = 0;
    for character in string.chars() {
        current = ((current + character as usize) * 17) % 256;
    }
    current
}

/// Part 1: Sum of instruction hashes
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for step in input.lines().next().ok_or("empty input")?.split(",") {
        sum += hash(step);
    }
    Ok(sum.to_string())
}

/// Part 2: Sum of focal powers after evaluation
///
/// Starting with 256 empty boxes each step either removes (dash) or adds (equal
/// sign) a lense to the box with its index equal to the label's hash value. If
/// a lense is added to a box that already holds a lense with the same label,
/// the lense is replaced at its place otherwise the new lense is added to the
/// end of the box. The focal power of a lense is the product of its focal
/// length, the one-based index of its box, and its one-based position inside
/// the box.
pub fn part2(input: String) -> crate::PuzzleResult {
    let re = Regex::new("(?<label>[a-z]+)(-|=(?<focal_length>[1-9]))").unwrap();
    let mut boxes = vec![Vec::new(); 256];
    for step in input.lines().next().ok_or("empty input")?.split(",") {
        let step = re.captures(step).ok_or(format!("invalid step {step}"))?;
        let label = step["label"].to_string();
        let index = hash(&label);
        match (
            step.name("focal_length")
                .map(|m| m.as_str().parse::<usize>().unwrap()),
            boxes[index].iter().position(|(s, _)| s == &label),
        ) {
            (Some(focal_length), Some(position)) => boxes[index][position].1 = focal_length,
            (Some(focal_length), None) => boxes[index].push((label, focal_length)),
            (None, Some(position)) => drop(boxes[index].remove(position)),
            _ => (),
        }
    }
    let mut sum = 0;
    for (index, lenses) in boxes.iter().enumerate() {
        for (position, (_, focal_length)) in lenses.iter().enumerate() {
            sum += (index + 1) * (position + 1) * focal_length;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "1320");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "145");
    }
}
