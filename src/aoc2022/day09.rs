//! # Day 9: Rope Bridge
//!
//! [puzzle site](https://adventofcode.com/2022/day/9)

fn solution<const ROPE_LENGTH: usize>(input: String) -> crate::PuzzleResult {
    let mut rope_positions = [[0i32, 0i32]; ROPE_LENGTH];
    let mut where_tail_was =
        std::collections::HashSet::from([rope_positions[ROPE_LENGTH - 1].clone()]);
    for line in input.lines() {
        let motion: Vec<&str> = line.split_whitespace().collect();
        let head_velocity = match motion[0] {
            "R" => [1, 0],
            "U" => [0, 1],
            "L" => [-1, 0],
            "D" => [0, -1],
            _ => panic!(),
        };
        let head_speed: u8 = motion[1].parse().unwrap();
        for _ in 0..head_speed {
            rope_positions[0][0] += head_velocity[0];
            rope_positions[0][1] += head_velocity[1];
            for k in 0..(ROPE_LENGTH - 1) {
                for [i, j] in [[0, 1], [1, 0]] {
                    if (rope_positions[k][i] - rope_positions[k + 1][i]).abs() == 2 {
                        rope_positions[k + 1][i] =
                            (rope_positions[k][i] + rope_positions[k + 1][i]) / 2;
                        if (rope_positions[k][j] - rope_positions[k + 1][j]).abs() == 2 {
                            rope_positions[k + 1][j] =
                                (rope_positions[k][j] + rope_positions[k + 1][j]) / 2;
                        } else {
                            rope_positions[k + 1][j] = rope_positions[k][j];
                        }
                    }
                }
            }
            where_tail_was.insert(rope_positions[ROPE_LENGTH - 1].clone());
        }
    }
    Ok(where_tail_was.len().to_string())
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    solution::<2>(input)
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    solution::<10>(input)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "13");
    }

    #[test]
    fn test_part2() {
        const INPUTS: [&str; 2] = [INPUT, "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"];
        const RESULTS: [&str; 2] = ["1", "36"];
        for i in 0..2 {
            assert_eq!(&super::part2(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }
}
