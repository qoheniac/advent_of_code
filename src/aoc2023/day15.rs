//! # Day 15: Lens Library
//!
//! [puzzle site](https://adventofcode.com/2023/day/15)

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for step in input.lines().next().ok_or("empty input")?.split(",") {
        let mut current = 0;
        for character in step.chars() {
            current = ((current + character as usize) * 17) % 256;
        }
        sum += current;
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
}
