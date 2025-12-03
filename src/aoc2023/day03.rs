//! # Day 3: Gear Ratios
//!
//! Any number adjacent (horizontally, vertically, or diagonally) to a symbol
//! that isn't a dot is a part number and any star that is adjacent to exactly
//! two part numbers is a gear with a gear ratio that is the product of the
//! adjacent part numbers.
//!
//! [puzzle site](https://adventofcode.com/2023/day/3)

/// Part 1: Sum up all part numbers
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    let mut previous_symbol_positions = Vec::new();
    let mut previous_candidates = Vec::<(usize, usize, usize)>::new();
    for line in input.lines() {
        let mut symbol_positions = Vec::new();
        let mut candidates = Vec::new();
        let mut growing_number: Option<(usize, String)> = None;

        // Collect part number candidates and symbol positions
        for (position, character) in line.chars().enumerate() {
            if character.is_numeric() {
                if let Some((_, string)) = growing_number.as_mut() {
                    // Extend number
                    string.push(character)
                } else {
                    // New number
                    growing_number = Some((position, character.to_string()))
                }
            } else {
                if let Some((start, string)) = &growing_number {
                    // End number
                    candidates.push((*start, position, string.parse()?));
                    growing_number = None
                }
                if character != '.' {
                    // Symbol
                    symbol_positions.push(position);
                }
            }
        }
        // End of line ends number
        if let Some((start, string)) = growing_number {
            candidates.push((start, line.chars().count() - 1, string.parse()?));
        }

        // Check if previous candidates are part numbers
        'candidate: for (start, end, number) in previous_candidates {
            for position in &symbol_positions {
                if (start.saturating_sub(1)..=end).contains(position) {
                    sum += number;
                    continue 'candidate;
                }
            }
        }

        // Check if current candidates are part numbers
        previous_candidates = Vec::new();
        'candidate: for (start, end, number) in candidates {
            for position in previous_symbol_positions.iter().chain(&symbol_positions) {
                if (start.saturating_sub(1)..=end).contains(position) {
                    sum += number;
                    continue 'candidate;
                }
            }

            // Remember symbol positions and remaining candidates
            previous_candidates.push((start, end, number))
        }
        previous_symbol_positions = symbol_positions;
    }
    Ok(sum.to_string())
}

/// Part 2: Sum up all gear ratios
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    let mut previous_stars = Vec::<(usize, Vec<usize>)>::new();
    let mut previous_numbers = Vec::<(usize, usize, usize)>::new();
    for line in input.lines() {
        let mut stars = Vec::new();
        let mut numbers = Vec::new();
        let mut number: Option<(usize, String)> = None;

        // Collect part number candidates and symbol positions
        for (position, character) in line.chars().enumerate() {
            if character.is_numeric() {
                if let Some((_, string)) = number.as_mut() {
                    // Extend number
                    string.push(character)
                } else {
                    // New number
                    number = Some((position, character.to_string()))
                }
            } else {
                if let Some((start, string)) = &number {
                    // End number
                    numbers.push((*start, position, string.parse()?));
                    number = None
                }
                if character == '*' {
                    // Star
                    stars.push((position, Vec::new()));
                }
            }
        }
        // End of line ends number
        if let Some((start, string)) = number {
            numbers.push((start, line.chars().count() - 1, string.parse()?));
        }

        // Check new stars against previous and new numbers
        for (position, neighbors) in &mut stars {
            for (start, end, number) in numbers.iter().chain(&previous_numbers) {
                if (start.saturating_sub(1)..=*end).contains(position) {
                    neighbors.push(*number);
                }
            }
        }

        // Check previous stars against new numbers
        for (position, neighbors) in &mut previous_stars {
            for (start, end, number) in &numbers {
                if (start.saturating_sub(1)..=*end).contains(position) {
                    neighbors.push(*number);
                }
            }

            // Add gear ratio if star is a gear
            if neighbors.len() == 2 {
                sum += neighbors[0] * neighbors[1];
            }
        }

        // Remember stars and numbers
        previous_stars = stars;
        previous_numbers = numbers;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598.."
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "4361");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "467835");
    }
}
