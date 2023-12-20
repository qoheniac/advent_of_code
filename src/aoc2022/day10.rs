//! # Day 10: Cathode-Ray Tube
//!
//! [puzzle site](https://adventofcode.com/2022/day/10)

enum Part {
    Part1,
    Part2,
}
use Part::*;

fn solution(input: String, part: Part) -> crate::PuzzleResult {
    let mut sprite_position: i32 = 1;
    let mut cycle: u16 = 0;
    let mut busy = false;
    let mut add = None;
    let mut sum = 0; // output for part 1
    let mut string = String::new(); // output for part 2
    let mut lines = input.lines();
    loop {
        if busy {
            busy = false;
        } else {
            if let Some(value) = add {
                sprite_position += value;
                add = None;
            }
            if let Some(line) = lines.next() {
                let mut cmd = line.split_whitespace();
                if cmd.next().unwrap() == "addx" {
                    let value: i32 = cmd.next().unwrap().parse().unwrap();
                    add = Some(value);
                    busy = true;
                }
            } else {
                break;
            }
        }
        let ray_position = cycle.rem_euclid(40);
        if ray_position == 0 {
            string.push('\n');
        }
        if ((sprite_position - 1)..=(sprite_position + 1)).contains(&(ray_position as i32)) {
            string.push('#');
        } else {
            string.push('.');
        }
        cycle += 1;
        if (cycle + 20).rem_euclid(40) == 0 {
            sum += cycle as i32 * sprite_position;
        }
    }
    Ok(match part {
        Part1 => sum.to_string(),
        Part2 => string,
    })
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Part1)
}
/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, Part2)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "addx 15\n",
        "addx -11\n",
        "addx 6\n",
        "addx -3\n",
        "addx 5\n",
        "addx -1\n",
        "addx -8\n",
        "addx 13\n",
        "addx 4\n",
        "noop\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx -35\n",
        "addx 1\n",
        "addx 24\n",
        "addx -19\n",
        "addx 1\n",
        "addx 16\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "addx 21\n",
        "addx -15\n",
        "noop\n",
        "noop\n",
        "addx -3\n",
        "addx 9\n",
        "addx 1\n",
        "addx -3\n",
        "addx 8\n",
        "addx 1\n",
        "addx 5\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -36\n",
        "noop\n",
        "addx 1\n",
        "addx 7\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "addx 6\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx 7\n",
        "addx 1\n",
        "noop\n",
        "addx -13\n",
        "addx 13\n",
        "addx 7\n",
        "noop\n",
        "addx 1\n",
        "addx -33\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 8\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx 17\n",
        "addx -9\n",
        "addx 1\n",
        "addx 1\n",
        "addx -3\n",
        "addx 11\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx -13\n",
        "addx -19\n",
        "addx 1\n",
        "addx 3\n",
        "addx 26\n",
        "addx -30\n",
        "addx 12\n",
        "addx -1\n",
        "addx 3\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -9\n",
        "addx 18\n",
        "addx 1\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "addx 9\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx -37\n",
        "addx 1\n",
        "addx 3\n",
        "noop\n",
        "addx 15\n",
        "addx -21\n",
        "addx 22\n",
        "addx -6\n",
        "addx 1\n",
        "noop\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx -10\n",
        "noop\n",
        "noop\n",
        "addx 20\n",
        "addx 1\n",
        "addx 2\n",
        "addx 2\n",
        "addx -6\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "noop"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "13140");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            super::part2(INPUT.to_string()).unwrap(),
            concat!(
                "\n##..##..##..##..##..##..##..##..##..##..",
                "\n###...###...###...###...###...###...###.",
                "\n####....####....####....####....####....",
                "\n#####.....#####.....#####.....#####.....",
                "\n######......######......######......####",
                "\n#######.......#######.......#######....."
            )
        );
    }
}
