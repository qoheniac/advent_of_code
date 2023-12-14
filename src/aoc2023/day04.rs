//! # Day 4: Scratchcards
//!
//! Each line holds a card with two lists of numbers and it's important how many
//! of the numbers in the second list match numbers in the first list.

/// Part 1: Total points
///
/// On each line the first match is worth one point and every further match
/// doubles the points for that line.
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut total = 0;
    for line in input.lines() {
        if let Some((winning, numbers)) = line
            .split_once(": ")
            .and_then(|split| split.1.split_once(" | "))
        {
            let winning: Vec<&str> = winning.split_whitespace().collect();
            let mut points = 0;
            for number in numbers.split_whitespace() {
                if winning.contains(&number) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2
                    }
                }
            }
            total += points;
        }
    }
    Ok(total.to_string())
}

/// Part 2: Total number of cards
///
/// Each card produces one copy of as many of the following cards as the card
/// has matches.
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut total = 0;
    let mut following_copies = std::collections::VecDeque::new();
    for line in input.lines() {
        if let Some((winning, numbers)) = line
            .split_once(": ")
            .and_then(|split| (split.1.split_once(" | ")))
        {
            let copies = 1 + following_copies.pop_front().unwrap_or(0);
            total += copies;
            let winning: Vec<&str> = winning.split_whitespace().collect();
            let mut pointer = 0;
            for number in numbers.split_whitespace() {
                if winning.contains(&number) {
                    if let Some(other_copies) = following_copies.get_mut(pointer) {
                        *other_copies += copies;
                    } else {
                        following_copies.push_back(copies);
                    }
                    pointer += 1;
                }
            }
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "30");
    }
}
