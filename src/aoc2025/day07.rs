//! # Day 7: Laboratories
//!
//! The input holds a map of empty spaces (.), beam splitters (^) and a tachyon
//! source position (S). A tachyon starts at S and only ever travels downwards.
//! When hitting a splitter the tachyon continues traveling downwards left or
//! right of the splitter.
//!
//! [puzzle site](https://adventofcode.com/2025/day/7)

fn solution(input: String) -> Result<[u64; 2], String> {
    let mut beam = std::collections::HashMap::new();
    let mut lines = input.lines();
    for (index, character) in lines.next().ok_or("empty input")?.chars().enumerate() {
        if character == 'S' {
            beam.insert(index, 1);
        }
    }
    let mut split_count = 0;
    for line in input.lines() {
        for (index, character) in line.chars().enumerate() {
            if character == '^' {
                if let Some(number) = beam.remove(&index) {
                    split_count += 1;
                    for new_index in [index - 1, index + 1] {
                        beam.entry(new_index)
                            .and_modify(|count| *count += number)
                            .or_insert(number);
                    }
                }
            }
        }
    }
    Ok([split_count, beam.into_values().sum::<u64>()])
}

/// Day 1: Number of splitters that can be hit
pub fn part1(input: String) -> crate::PuzzleResult {
    Ok(solution(input)?[0].to_string())
}

/// Day 2: Number of possible paths
pub fn part2(input: String) -> crate::PuzzleResult {
    Ok(solution(input)?[1].to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "40");
    }
}
