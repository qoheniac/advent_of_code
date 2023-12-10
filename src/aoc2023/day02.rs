//! Day 2: Cube Conundrum
//!
//! Each line holds a game ID number (like the 11 in Game 11: ...) followed by a
//! semicolon-separated list of subsets of red, green, or blue cubes that were
//! revealed from a bag (like 3 red, 5 green, 4 blue).

/// Part 1: Sum up IDs of possible games if the bag contained 12 red, 13 green,
/// and 14 blue cubes
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    'lines: for line in input.lines() {
        if let Some((game, subsets)) = line.split_once(": ") {
            let id: usize = match game.split_once(" ") {
                Some(("Game", number)) => number.parse()?,
                _ => Err(format!("no game ID found in {line}"))?,
            };
            for cubes in subsets.split("; ").flat_map(|subset| subset.split(", ")) {
                if match cubes.split_once(" ") {
                    Some((number, "red")) => number.parse::<usize>()? > 12,
                    Some((number, "green")) => number.parse::<usize>()? > 13,
                    Some((number, "blue")) => number.parse::<usize>()? > 14,
                    _ => Err(format!("invalid subset in {line}"))?,
                } {
                    continue 'lines;
                }
            }
            sum += id;
        }
    }
    Ok(sum.to_string())
}

/// Part 2: Sum up over all games the products of the minimum number of cubes
/// needed of each color
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut sum = 0;
    for line in input.lines() {
        let (mut reds, mut greens, mut blues) = (0, 0, 0);
        if let Some(subsets) = line.split(": ").last() {
            for cubes in subsets.split("; ").flat_map(|subset| subset.split(", ")) {
                match cubes.split_once(" ") {
                    Some((number, "red")) => reds = reds.max(number.parse()?),
                    Some((number, "green")) => greens = greens.max(number.parse()?),
                    Some((number, "blue")) => blues = blues.max(number.parse()?),
                    _ => Err(format!("invalid subset in {line}"))?,
                }
            }
        }
        sum += reds * greens * blues;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "8");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "2286");
    }
}
