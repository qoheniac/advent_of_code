//! # Day 1: Calorie Counting
//!
//! [puzzle site](https://adventofcode.com/2022/day/1)

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut max_calories = 0;
    for elf in input.split("\n\n") {
        let mut sum = 0;
        for line in elf.lines() {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
        if sum > max_calories {
            max_calories = sum;
        }
    }
    Ok(max_calories.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut top_calories = vec![0, 0, 0];
    for elf in input.split("\n\n") {
        let mut sum = 0;
        for line in elf.lines() {
            let calories: i32 = line.parse().unwrap();
            sum += calories;
        }
        top_calories.push(sum);
        top_calories.sort();
        top_calories.remove(0);
    }
    Ok(top_calories.iter().sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
        1000\n\
        2000\n\
        3000\n\
            \n\
        4000\n\
            \n\
        5000\n\
        6000\n\
            \n\
        7000\n\
        8000\n\
        9000\n\
            \n\
        10000";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "24000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "45000");
    }
}
