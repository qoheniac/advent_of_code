//! # Day 1: Historian Hysteria
//!
//! The two columns are lists of integer IDs.
//!
//! [puzzle site](https://adventofcode.com/2024/day/1)

use std::{error::Error, iter::zip};

fn parse_lists(input: String) -> Result<[Vec<usize>; 2], Box<dyn Error>> {
    let mut list1 = Vec::<usize>::new();
    let mut list2 = Vec::<usize>::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        list1.push(split.next().ok_or("empty line")?.parse()?);
        list2.push(split.next().ok_or("no second id")?.parse()?);
    }
    Ok([list1, list2])
}

/// Part 1: Total sum of differences between sorted lists
pub fn part1(input: String) -> crate::PuzzleResult {
    let [mut list1, mut list2] = parse_lists(input)?;
    list1.sort_unstable();
    list2.sort_unstable();
    let mut sum = 0;
    for (id1, id2) in zip(list1, list2) {
        sum += id1.abs_diff(id2);
    }
    Ok(sum.to_string())
}

/// Part 2: Sum all products of list 1 IDs with their multiplicity in list 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let [list1, list2] = parse_lists(input)?;
    let mut sum = 0;
    for id1 in list1 {
        sum += id1 * list2.iter().filter(|id2| id1.eq(id2)).count();
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "31");
    }
}
