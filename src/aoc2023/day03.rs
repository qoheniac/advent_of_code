//! Day 3: Gear Ratios
//!
//! Any number adjacent (horizontally, vertically, or diagonally) to a symbol
//! that isn't a dot is a part number.

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
                if (start.checked_sub(1).unwrap_or(0)..=end).contains(&position) {
                    sum += number;
                    continue 'candidate;
                }
            }
        }

        // Check if current candidates are part numbers
        previous_candidates = Vec::new();
        'candidate: for (start, end, number) in candidates {
            for position in previous_symbol_positions.iter().chain(&symbol_positions) {
                if (start.checked_sub(1).unwrap_or(0)..=end).contains(&position) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598.."
            .to_string();
        assert_eq!(&super::part1(input).unwrap(), "4361");
    }
}
