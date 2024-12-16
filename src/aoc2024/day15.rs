//! # Day 15: Warehouse Woes
//!
//! The input contains a map of a warehouse and instructions separated by a
//! blank line. The map is rectangular and surrounded by walls (#). On the
//! inside there might be walls too as well as boxes (O). There is also a robot
//! (@) moving according to the instructions while shifting boxes as necessary.
//! Walls cannot be shifted and instructions are ignored if a wall prevents the
//! motion. The instructions can be right (>), up (^), left (<), and down(v).
//! Newlines in the list of instruction may be ignored. The results is the
//!
//! [puzzle site](https://adventofcode.com/2024/day15)
#[derive(Clone, Copy)]
enum Instruction {
    Right,
    Up,
    Left,
    Down,
}

use std::collections::HashSet;

use Instruction::*;

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '>' => Right,
            '^' => Up,
            '<' => Left,
            'v' => Down,
            _ => return Err(format!("{c} is no direction")),
        })
    }
}

struct Warehouse {
    width: usize,
    height: usize,
    robot: [usize; 2],
    walls: HashSet<[usize; 2]>,
    boxes: HashSet<[usize; 2]>,
}

impl Warehouse {
    fn apply_instruction_to_location(
        &self,
        instruction: Instruction,
        [i, j]: [usize; 2],
    ) -> Option<[usize; 2]> {
        Some(match instruction {
            Right if j + 1 < self.width => [i, j + 1],
            Up if i > 0 => [i - 1, j],
            Left if j > 0 => [i, j - 1],
            Down if i + 1 < self.height => [i + 1, j],
            _ => return None,
        })
    }

    fn instruct(&mut self, instruction: Instruction) {
        let mut locations = vec![self.robot];
        while let Some(location) =
            self.apply_instruction_to_location(instruction, *locations.last().unwrap())
        {
            if self.walls.contains(&location) {
                break;
            }
            locations.push(location);
            if !self.boxes.contains(&location) {
                if locations.len() > 2 {
                    self.boxes.remove(&locations[1]);
                    self.boxes.insert(location);
                }
                self.robot = locations[1];
                break;
            }
        }
    }
}

impl std::str::FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = (s.lines().next())
            .and_then(|l| l.chars().count().checked_sub(2))
            .ok_or("input width too small")?;
        let height = (s.lines().count().checked_sub(2)).ok_or("input height too small")?;
        let mut robot = None;
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        for (i, line) in s.lines().enumerate() {
            if [0, height + 1].contains(&i) {
                continue;
            }
            for (j, character) in line.chars().enumerate() {
                if [0, width + 1].contains(&j) {
                    continue;
                }
                match character {
                    '@' => {
                        if robot.is_none() {
                            robot = Some([i - 1, j - 1])
                        } else {
                            return Err("more than one robot".to_owned());
                        }
                    }
                    '#' => {
                        walls.insert([i - 1, j - 1]);
                    }
                    'O' => {
                        boxes.insert([i - 1, j - 1]);
                    }
                    _ => (),
                }
            }
        }
        Ok(Self {
            width,
            height,
            robot: robot.ok_or("no robot found")?,
            walls,
            boxes,
        })
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let (warehouse, instructions) = input.split_once("\n\n").ok_or("no blank line found")?;
    let mut warehouse: Warehouse = warehouse.parse()?;
    for instruction in instructions.lines().flat_map(|l| l.chars()) {
        warehouse.instruct(instruction.try_into()?);
    }
    Ok((warehouse.boxes.iter())
        .map(|&[i, j]| 100 * (i + 1) + j + 1)
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    const SMALL: &str = concat!(
        "########\n",
        "#..O.O.#\n",
        "##@.O..#\n",
        "#...O..#\n",
        "#.#.O..#\n",
        "#...O..#\n",
        "#......#\n",
        "########\n",
        "\n",
        "<^^>>>vv<v>>v<<",
    );

    const BIG: &str = concat!(
        "##########\n",
        "#..O..O.O#\n",
        "#......O.#\n",
        "#.OO..O.O#\n",
        "#..O@..O.#\n",
        "#O#..O...#\n",
        "#O..O..O.#\n",
        "#.OO.O.OO#\n",
        "#....O...#\n",
        "##########\n",
        "\n",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(SMALL.to_string()).unwrap(), "2028");
        assert_eq!(&super::part1(BIG.to_string()).unwrap(), "10092");
    }
}
