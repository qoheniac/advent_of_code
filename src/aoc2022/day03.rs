//! # Day 3: Rucksack Reorganization
//!
//! [puzzle site](https://adventofcode.com/2022/day/3)

fn priority(item: char) -> u32 {
    let mut priority = item.to_digit(36).unwrap() - 9;
    if item.is_ascii_uppercase() {
        priority += 26
    }
    priority
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let mid_index = line.len() / 2;
        let compartment1 = &line[..mid_index];
        let compartment2 = &line[mid_index..];
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                sum += priority(item);
                break;
            }
        }
    }
    Ok(sum.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    let mut rucksacks = input.lines();
    loop {
        let rucksack1;
        match rucksacks.next() {
            Some(rucksack) => rucksack1 = rucksack,
            None => break,
        }
        let rucksack2 = rucksacks.next().unwrap();
        let rucksack3 = rucksacks.next().unwrap();
        for item in rucksack1.chars() {
            if rucksack2.contains(item) & rucksack3.contains(item) {
                sum += priority(item);
                break;
            }
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "vJrwpWtwJgWrhcsFMMfFFhFp\n",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
        "PmmdzqPrVvPwwTWBwg\n",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
        "ttgJtRGJQctTZtZT\n",
        "CrZsJsPPZsGzwwsLwLmpwMDw"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "157");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "70");
    }
}
