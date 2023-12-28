//! # Day 14: Parabolic Reflector Dish
//!
//! The input holds a rectangular map of round stones (O), cubic stones (#) and
//! empty space (.) on a platform. Tilting the platform north, west, south, or
//! east lets all round stones roll as far as possible in the respective
//! direction. The goal is to find the total load, where each round stone adds a
//! load equal to its distance from the southern end.
//!
//! [puzzle site](https://adventofcode.com/2023/day/14)

#[derive(Clone, Copy)]
struct Chain {
    start: usize,
    length: usize,
}

impl Chain {
    fn load(&self, height: usize) -> usize {
        (1 + 2 * (height - self.start) - self.length) * self.length / 2
    }
}

/// Part 1: Tilt north
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut lines = input.lines();
    let mut chains = Vec::new();
    let mut chains_in_progress = Vec::new();
    for character in lines.next().ok_or("empty input")?.chars() {
        match character {
            'O' => chains_in_progress.push(Chain {
                start: 0,
                length: 1,
            }),
            '#' => chains_in_progress.push(Chain {
                start: 1,
                length: 0,
            }),
            _ => chains_in_progress.push(Chain {
                start: 0,
                length: 0,
            }),
        }
    }
    let mut height = 1;
    for line in lines {
        height += 1;
        for (column, character) in line.chars().enumerate() {
            let chain = &mut chains_in_progress[column];
            match character {
                'O' => chain.length += 1,
                '#' => {
                    if chain.length > 0 {
                        chains.push(*chain)
                    }
                    *chain = Chain {
                        start: height,
                        length: 0,
                    };
                }
                _ => (),
            }
        }
    }
    chains.append(&mut chains_in_progress);
    let mut sum = 0;
    for chain in chains {
        sum += chain.load(height);
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "136");
    }
}
