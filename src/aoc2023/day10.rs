//! # Day 10: Pipe Maze
//!
//! The input contains a grid of tiles with one starting tile on a pipe loop.
//!
//! [puzzle site](https://adventofcode.com/2023/day/10)

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

#[derive(Clone, Copy, Debug)]
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
        // try all tiles
        for tile in TILES {
            self.tiles[i][j] = tile;
            // try all directions
            for direction in DIRECTIONS {
                let state = State {
                    location: (i, j),
                    direction,
                };
                // check if direction fits tile
                if let Ok(turned_around) = self.try_turn_around(state) {
                    // check if neighbors are reachable
                    if self.try_step(state).is_ok() && self.try_step(turned_around).is_ok() {
                        return Ok(state);
                    }
                }
            }
        }
        // reset tile if no fitting tile was found
        self.tiles[i][j] = original_tile;
        Err(format!("no tile fits at ({i}, {j})"))
    }
}

fn parse_input(input: String) -> Result<(Grid, State), String> {
    // fill grid and find start location
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
    // replace starting tile and get initial state
    let state = grid.try_fitting_tile(start_location.ok_or("start not found")?)?;
    Ok((grid, state))
}

/// Part 1: Half length of the loop
pub fn part1(input: String) -> crate::PuzzleResult {
    let (grid, original_state) = parse_input(input)?;
    let mut state = original_state.clone();
    for i in 1.. {
        state = grid.try_step(state)?;
        if state == original_state {
            return Ok((i / 2).to_string());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        const INPUTS: [&str; 2] = [
            ".....\n.S-7.\n.|.|.\n.L-J.\n.....",
            "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...",
        ];
        const RESULTS: [&str; 2] = ["4", "8"];
        for i in 0..2 {
            assert_eq!(&super::part1(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }
}
