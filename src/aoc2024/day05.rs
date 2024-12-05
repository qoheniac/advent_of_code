//! # Day 5: Print Queue
//!
//! The input consists of two sections separated by a blank line. The first
//! section is a set of rules, one on each line, of the form X|Y, meaning X
//! before Y. The second section holds lines of comma-separated sequences of
//! numbers.
//!
//! [puzzle site](https://adventofcode.com/2024/day05)

use std::{collections::HashMap, error::Error};

fn parse_rules(rules: &str) -> Result<HashMap<u16, Vec<u16>>, Box<dyn Error>> {
    let mut map = HashMap::new();
    for line in rules.lines() {
        let (before, after) = line.split_once("|").ok_or("rule without |")?;
        let after = after.parse()?;
        map.entry(before.parse()?)
            .and_modify(|v: &mut Vec<_>| v.push(after))
            .or_insert(vec![after]);
    }
    Ok(map)
}

/// Part 1: Sum middle numbers over all correctly ordered sequences
pub fn part1(input: String) -> crate::PuzzleResult {
    let (rules, sequences) = input.split_once("\n\n").ok_or("no blank line found")?;
    let rules = parse_rules(rules)?;
    let mut sum = 0;
    'sequences: for sequence in sequences.lines() {
        let mut numbers_before = Vec::new();
        for number in sequence.split(",") {
            let number: u16 = number.parse()?;
            if let Some(numbers_after) = rules.get(&number) {
                for number_before in &numbers_before {
                    if numbers_after.contains(number_before) {
                        continue 'sequences;
                    }
                }
            }
            numbers_before.push(number);
        }
        sum += numbers_before[(numbers_before.len() - 1) / 2];
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "47|53\n",
        "97|13\n",
        "97|61\n",
        "97|47\n",
        "75|29\n",
        "61|13\n",
        "75|53\n",
        "29|13\n",
        "97|29\n",
        "53|29\n",
        "61|53\n",
        "97|53\n",
        "61|29\n",
        "47|13\n",
        "75|47\n",
        "97|75\n",
        "47|61\n",
        "75|61\n",
        "47|29\n",
        "75|13\n",
        "53|13\n",
        "\n",
        "75,47,61,53,29\n",
        "97,61,53,29,13\n",
        "75,29,13\n",
        "75,97,47,61,53\n",
        "61,13,29\n",
        "97,13,75,29,47"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "143");
    }
}
