//! # Day 17: Clumsy Crucible
//!
//! [puzzle site](https://adventofcode.com/2023/day/17)

use ndarray::{Array2, Dim};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location(i32, i32);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

impl Direction {
    fn step(&self, Location(i, j): Location) -> Location {
        match self {
            &Down => Location(i + 1, j),
            &Left => Location(i, j - 1),
            &Right => Location(i, j + 1),
            &Up => Location(i - 1, j),
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            &Down => Right,
            &Left => Down,
            &Right => Up,
            &Up => Left,
        }
    }

    fn turn_right(&self) -> Direction {
        self.turn_left().turn_left().turn_left()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct State(Location, Direction, u8);

impl State {
    fn neighbors(&self) -> Vec<State> {
        let &State(location, direction, count) = self;
        let mut states = Vec::new();
        if count < 2 {
            states.push(State(direction.step(location), direction, count + 1));
        }
        for direction in [direction.turn_left(), direction.turn_right()] {
            states.push(State(direction.step(location), direction, 0));
        }
        states
    }
}

struct HeatMap(Array2<u32>);

impl HeatMap {
    fn heat(&self, Location(i, j): Location) -> Option<u32> {
        self.0.get((i as usize, j as usize)).copied()
    }
}

impl std::str::FromStr for HeatMap {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heats = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            height += 1;
            width = 0;
            for c in line.chars() {
                width += 1;
                heats.push(c.to_digit(10).ok_or(format!("{c} is not a digit"))?)
            }
        }
        Ok(HeatMap(Array2::from_shape_vec((height, width), heats)?))
    }
}

struct Losses(Array2<[u32; 10]>);

impl Losses {
    fn new(dim: Dim<[usize; 2]>) -> Self {
        Losses(Array2::from_elem(dim, [u32::MAX; 10]))
    }

    fn index(direction: Direction, count: u8) -> usize {
        if count < 2 {
            count as usize
                + match direction {
                    Down => 0,
                    Left => 2,
                    Right => 4,
                    Up => 6,
                }
        } else {
            match direction {
                Down | Up => 8,
                Left | Right => 9,
            }
        }
    }

    fn get(&self, State(Location(i, j), direction, count): State) -> Option<u32> {
        self.0
            .get((i as usize, j as usize))
            .and_then(|array| Some(array[Self::index(direction, count)]))
    }

    fn update(&mut self, State(Location(i, j), direction, count): State, loss: u32) {
        if let Some(array) = self.0.get_mut((i as usize, j as usize)) {
            let index = Self::index(direction, count);
            array[index] = array[index].min(loss);
        }
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let map: HeatMap = input.parse()?;
    let mut losses = Losses::new(map.0.raw_dim());
    let mut to_visit = std::collections::HashSet::new();
    for first_step in [
        State(Location(0, 1), Right, 0),
        State(Location(1, 0), Down, 0),
    ] {
        losses.update(first_step, map.heat(first_step.0).unwrap());
        to_visit.insert(first_step);
    }
    let destination = Location(map.0.nrows() as i32 - 1, map.0.ncols() as i32 - 1);
    let optimal_loss = 'dijkstra: loop {
        let best = to_visit
            .iter()
            .min_by_key(|&&state| losses.get(state).unwrap())
            .copied()
            .unwrap();
        to_visit.remove(&best);
        let best_loss = losses.get(best).unwrap();
        for neighbor in best.neighbors() {
            if let Some(extra_loss) = map.heat(neighbor.0) {
                let neighbor_loss = best_loss + extra_loss;
                if neighbor.0 == destination {
                    break 'dijkstra neighbor_loss;
                }
                if losses.get(neighbor) == Some(u32::MAX) {
                    to_visit.insert(neighbor);
                }
                losses.update(neighbor, neighbor_loss);
            }
        }
    };
    Ok(optimal_loss.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "102");
    }
}
