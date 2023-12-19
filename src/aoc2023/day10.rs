//! # Day 10: Pipe Maze
//!
//! The input contains a grid of tiles with one starting tile on a pipe loop.
//!
//! [puzzle site](https://adventofcode.com/2023/day/10)

use num::Integer; // check whether number is odd

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;
const DIRECTIONS: [Direction; 4] = [Down, Left, Right, Up];

impl Direction {
    fn turn_around(&self) -> Self {
        match self {
            Down => Up,
            Left => Right,
            Right => Left,
            Up => Down,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Ground,
    Horizontal,
    Vertical,
}
use Tile::*;
const TILES: [Tile; 7] = [BendNE, BendNW, BendSE, BendSW, Ground, Horizontal, Vertical];

impl Tile {
    fn try_from_char(character: char) -> Result<Self, String> {
        Ok(match character {
            'L' => BendNE,
            'J' => BendNW,
            'F' => BendSE,
            '7' => BendSW,
            '.' => Ground,
            '-' => Horizontal,
            '|' => Vertical,
            _ => Err(format!("{character} is not a tile"))?,
        })
    }

    fn try_follow(&self, direction: Direction) -> Result<Direction, String> {
        Ok(match (self, direction) {
            (BendNE, Down) => Right,
            (BendNE, Left) => Up,
            (BendNW, Down) => Left,
            (BendNW, Right) => Up,
            (BendSW, Right) => Down,
            (BendSW, Up) => Left,
            (BendSE, Left) => Down,
            (BendSE, Up) => Right,
            (Horizontal, Left) => Left,
            (Horizontal, Right) => Right,
            (Vertical, Down) => Down,
            (Vertical, Up) => Up,
            (tile, direction) => Err(format!("cannot enter {tile:?} moving {direction:?}"))?,
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
struct State {
    location: (usize, usize),
    direction: Direction,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn try_step(
        &self,
        State {
            location: (i, j),
            direction,
        }: State,
    ) -> Result<State, String> {
        let (i, j) = match direction {
            Down if i + 1 < self.height => (i + 1, j),
            Left if j > 0 => (i, j - 1),
            Right if j + 1 < self.width => (i, j + 1),
            Up if i > 0 => (i - 1, j),
            _ => Err(format!(
                "{direction:?} from ({i}, {j}) would leave the grid"
            ))?,
        };
        Ok(State {
            location: (i, j),
            direction: self.tiles[i][j].try_follow(direction)?,
        })
    }

    fn try_turn_around(
        &self,
        State {
            location: (i, j),
            direction,
        }: State,
    ) -> Result<State, String> {
        Ok(State {
            location: (i, j),
            direction: self.tiles[i][j].try_follow(direction.turn_around())?,
        })
    }
    fn try_fitting_tile(&mut self, (i, j): (usize, usize)) -> Result<State, String> {
        let original_tile = self.tiles[i][j];
        // Try all tiles
        for tile in TILES {
            self.tiles[i][j] = tile;
            // Try all directions
            for direction in DIRECTIONS {
                let state = State {
                    location: (i, j),
                    direction,
                };
                // Check if direction fits tile
                if let Ok(turned_around) = self.try_turn_around(state) {
                    // Check if neighbors are reachable
                    if self.try_step(state).is_ok() && self.try_step(turned_around).is_ok() {
                        return Ok(state);
                    }
                }
            }
        }
        // Reset tile if no fitting tile was found
        self.tiles[i][j] = original_tile;
        Err(format!("no tile fits at ({i}, {j})"))
    }
}

fn parse_input(input: String) -> Result<(Grid, Vec<State>), String> {
    // Fill grid and find start location
    let mut tiles = Vec::new();
    let mut start_location = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, character) in line.chars().enumerate() {
            row.push(if character == 'S' {
                start_location = Some((i, j));
                Ground // dummy tile (will be replace when the full grid is known)
            } else {
                Tile::try_from_char(character)?
            })
        }
        tiles.push(row);
    }
    let width = tiles[0].len();
    let height = tiles.len();
    let mut grid = Grid {
        tiles,
        width,
        height,
    };
    // Replace starting tile with fitting tile and get intial state
    let original_state = grid.try_fitting_tile(start_location.ok_or("start not found")?)?;

    // Collect states along the pipe
    let mut pipe = vec![original_state];
    loop {
        let state = grid.try_step(pipe.last().unwrap().clone())?;
        if state == original_state {
            return Ok((grid, pipe));
        }
        pipe.push(state);
    }
}

/// Part 1: Half length of the pipe
pub fn part1(input: String) -> crate::PuzzleResult {
    Ok((parse_input(input)?.1.len() / 2).to_string())
}

/// Part 2: Points enclosed by the pipe
pub fn part2(input: String) -> crate::PuzzleResult {
    let (grid, pipe) = parse_input(input)?;
    let locations: Vec<(usize, usize)> = pipe.iter().map(|state| state.location).collect();
    let mut inside_count = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            // skip tiles on the pipe
            if locations.contains(&(i, j)) {
                continue;
            }
            // diagonal ray casting to avoid going parallel to pipe
            let length = (grid.height - i).min(grid.width - j);
            let mut crossings_count = 0;
            for d in 1..length {
                let (m, n) = (i + d, j + d);
                // count crossing if pipe does not bend away from ray
                if ![BendNE, BendSW].contains(&grid.tiles[m][n]) && locations.contains(&(m, n)) {
                    crossings_count += 1;
                }
            }
            if crossings_count.is_odd() {
                inside_count += 1;
            }
        }
    }
    Ok(inside_count.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        const INPUTS: [&str; 2] = [
            ".....\n.S-7.\n.|.|.\n.L-J.\n.....",
            "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...",
        ];
        const RESULTS: [&str; INPUTS.len()] = ["4", "8"];
        for i in 0..INPUTS.len() {
            assert_eq!(&super::part1(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }

    #[test]
    fn test_part2() {
        const INPUTS: [&str; 3] = [
            concat!(
                "...........\n",
                ".S-------7.\n",
                ".|F-----7|.\n",
                ".||.....||.\n",
                ".||.....||.\n",
                ".|L-7.F-J|.\n",
                ".|..|.|..|.\n",
                ".L--J.L--J.\n",
                "..........."
            ),
            concat!(
                ".F----7F7F7F7F-7....\n",
                ".|F--7||||||||FJ....\n",
                ".||.FJ||||||||L7....\n",
                "FJL7L7LJLJ||LJ.L-7..\n",
                "L--J.L7...LJS7F-7L7.\n",
                "....F-J..F7FJ|L7L7L7\n",
                "....L7.F7||L7|.L7L7|\n",
                ".....|FJLJ|FJ|F7|.LJ\n",
                "....FJL-7.||.||||...\n",
                "....L---J.LJ.LJLJ..."
            ),
            concat!(
                "FF7FSF7F7F7F7F7F---7\n",
                "L|LJ||||||||||||F--J\n",
                "FL-7LJLJ||||||LJL-77\n",
                "F--JF--7||LJLJ7F7FJ-\n",
                "L---JF-JLJ.||-FJLJJ7\n",
                "|F|F-JF---7F7-L7L|7|\n",
                "|FFJF7L7F-JF7|JL---7\n",
                "7-L-JL7||F7|L7F-7F7|\n",
                "L.L7LFJ|||||FJL7||LJ\n",
                "L7JLJL-JLJLJL--JLJ.L"
            ),
        ];
        const RESULTS: [&str; INPUTS.len()] = ["4", "8", "10"];
        for i in 0..INPUTS.len() {
            assert_eq!(&super::part2(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }
}
