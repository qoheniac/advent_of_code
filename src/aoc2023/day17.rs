//! # Day 17: Clumsy Crucible
//!
//! The input consists of a city map where each number represents the heat a
//! crucible loses when it moves to that tile. The crucibles need to move a
//! minimum distance forward before they can turn left or right and after a
//! maximum distance they need to turn. The goal is to find the minimum heat
//! loss for a crucible starting in the top-left corner to reach the
//! bottom-right corner.
//!
//! [puzzle site](https://adventofcode.com/2023/day/17)

use ndarray::{Array2, Dim};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location(i32, i32);

const ORIGIN: Location = Location(0, 0);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}
use Direction::*;

impl Direction {
    fn step(&self, Location(i, j): Location, d: usize) -> Location {
        let d = d as i32;
        match *self {
            Down => Location(i + d, j),
            Left => Location(i, j - d),
            Right => Location(i, j + d),
            Up => Location(i - d, j),
        }
    }

    fn path(&self, location: Location, distance: usize) -> Vec<Location> {
        let mut path = Vec::new();
        for d in 1..=distance {
            path.push(self.step(location, d));
        }
        path
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Down => Right,
            Left => Down,
            Right => Up,
            Up => Left,
        }
    }

    fn turn_right(&self) -> Direction {
        self.turn_left().turn_left().turn_left()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct State<const MIN: usize, const MAX: usize>(Location, Direction, usize);

impl<const MIN: usize, const MAX: usize> State<MIN, MAX> {
    fn neighbors(&self) -> Vec<(Vec<Location>, Self)> {
        let &Self(location, direction, count) = self;
        let mut states = Vec::new();
        if count + MIN < MAX {
            let path = direction.path(location, 1);
            let location = *path.last().unwrap();
            states.push((path, Self(location, direction, count + 1)));
        }
        for direction in [direction.turn_left(), direction.turn_right()] {
            let path = direction.path(location, MIN);
            let location = *path.last().unwrap();
            states.push((path, Self(location, direction, 0)));
        }
        states
    }
}

struct HeatMap(Array2<u32>);

impl HeatMap {
    fn heat(&self, Location(i, j): Location) -> Option<u32> {
        self.0.get((i as usize, j as usize)).copied()
    }

    fn collect_heat(&self, path: Vec<Location>) -> Option<u32> {
        path.iter().map(|&location| self.heat(location)).sum()
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

struct Losses<const MIN: usize, const MAX: usize>(Array2<Vec<u32>>);

impl<const MIN: usize, const MAX: usize> Losses<MIN, MAX> {
    fn new(dim: Dim<[usize; 2]>) -> Self {
        Losses(Array2::from_elem(dim, vec![u32::MAX; 4 * (MAX - MIN) + 2]))
    }

    fn index(direction: Direction, count: usize) -> usize {
        if count + MIN < MAX {
            count
                + (MAX - MIN)
                    * match direction {
                        Down => 0,
                        Left => 1,
                        Right => 2,
                        Up => 3,
                    }
        } else {
            4 * (MAX - MIN)
                + match direction {
                    Down | Up => 0,
                    Left | Right => 1,
                }
        }
    }

    fn get(&self, State(Location(i, j), direction, count): State<MIN, MAX>) -> Option<u32> {
        self.0
            .get((i as usize, j as usize))
            .map(|array| array[Self::index(direction, count)])
    }

    fn update(&mut self, State(Location(i, j), direction, count): State<MIN, MAX>, loss: u32) {
        if let Some(array) = self.0.get_mut((i as usize, j as usize)) {
            let index = Self::index(direction, count);
            array[index] = array[index].min(loss);
        }
    }
}

fn solution<const MIN: usize, const MAX: usize>(input: String) -> crate::PuzzleResult {
    let map: HeatMap = input.parse()?;
    let mut losses = Losses::<MIN, MAX>::new(map.0.raw_dim());
    let mut to_visit = std::collections::HashSet::new();
    for initial_state in [State(ORIGIN, Left, MAX), State(ORIGIN, Up, MAX)] {
        losses.update(initial_state, 0);
        to_visit.insert(initial_state);
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
        for (path, neighbor) in best.neighbors() {
            if let Some(extra_loss) = map.collect_heat(path) {
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

/// Part 1: Forward steps mustn't be more than 3 before turn
pub fn part1(input: String) -> crate::PuzzleResult {
    solution::<1, 3>(input)
}

/// Part 2: Forward steps must be between 4 and 10 before turn
pub fn part2(input: String) -> crate::PuzzleResult {
    solution::<4, 10>(input)
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

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "94");
        let input = "111111111111\n999999999991\n999999999991\n999999999991\n999999999991";
        assert_eq!(&super::part2(input.to_string()).unwrap(), "71");
    }
}
