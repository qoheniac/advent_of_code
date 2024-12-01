//! # Day 1: Historian Hysteria
//!
//! Differences of sorted two columns are distances.
//!
//! [puzzle site](https://adventofcode.com/2024/day/1)

use std::iter::zip;

/// Part 1: Total distance
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut list1 = Vec::<usize>::new();
    let mut list2 = Vec::<usize>::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().ok_or("empty line")?.parse()?);
        list2.push(split.next().ok_or("no second id")?.parse()?);
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let mut sum = 0;
    for (id1, id2) in zip(list1, list2) {
        sum += id1.abs_diff(id2);
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(&super::part1(input.to_string()).unwrap(), "11");
    }
}
