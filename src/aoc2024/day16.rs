//! # Day 16: Reindeer Maze
//!
//! The input shows a maze of empty (.) tiles and walls (#) as well as a start
//! (S) and end (E) position. The direction at the start is east and taking a
//! step increases the score by 1 while turning by 90° increases the score by
//! 1000.
//!
//! [puzzle site](https://adventofcode.com/2024/day16)

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
}
use Tile::*;

impl Tile {
    fn try_from_char(character: char) -> Result<Self, String> {
        Ok(match character {
            '.' | 'S' | 'E' => Self::Empty,
            '#' => Self::Wall,
            _ => Err(format!("{character} is not a valid tile"))?,
        })
    }
}

struct Maze {
    width: usize,
    height: usize,
    map: Vec<Vec<Tile>>,
    start: [usize; 2],
    end: [usize; 2],
}

impl std::str::FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, character) in line.chars().enumerate() {
                if character == 'S' {
                    start = Some([i, j]);
                }
                if character == 'E' {
                    end = Some([i, j]);
                }
                row.push(Tile::try_from_char(character)?);
            }
            map.push(row);
        }
        Ok(Self {
            width: map[0].len(),
            height: map.len(),
            map,
            start: start.ok_or("no start found".to_owned())?,
            end: end.ok_or("no end found".to_owned())?,
        })
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    East,
    North,
    West,
    South,
}
use Direction::*;

impl Direction {
    fn neighbors(&self) -> [Direction; 2] {
        match self {
            East => [South, North],
            North => [East, West],
            West => [North, South],
            South => [West, East],
        }
    }
}

impl From<Direction> for usize {
    fn from(direction: Direction) -> Self {
        match direction {
            East => 0,
            North => 1,
            West => 2,
            South => 3,
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct MazeState {
    position: [usize; 2],
    direction: Direction,
}

impl From<([usize; 2], Direction)> for MazeState {
    fn from((position, direction): ([usize; 2], Direction)) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl Maze {
    fn initial_state(&self) -> MazeState {
        (self.start, East).into()
    }

    fn step(
        &self,
        MazeState {
            position: [i, j],
            direction,
        }: MazeState,
    ) -> Option<MazeState> {
        let position = match direction {
            East if j + 1 < self.width && self.map[i][j + 1] == Empty => [i, j + 1],
            North if i > 0 && self.map[i - 1][j] == Empty => [i - 1, j],
            West if j > 0 && self.map[i][j - 1] == Empty => [i, j - 1],
            South if i + 1 < self.height && self.map[i + 1][j] == Empty => [i + 1, j],
            _ => return None,
        };
        Some((position, direction).into())
    }

    fn neighbor_states(&self, state: MazeState) -> Vec<(MazeState, usize)> {
        let mut neighbors = Vec::from(
            (state.direction.neighbors())
                .map(|direction| ((state.position, direction).into(), 1000)),
        );
        if let Some(state) = self.step(state) {
            neighbors.push((state, 1));
        }
        neighbors
    }
}

struct Scores(Vec<Vec<[usize; 4]>>);

impl Scores {
    fn get(&self, state: MazeState) -> usize {
        self.0[state.position[0]][state.position[1]][usize::from(state.direction)]
    }

    fn set(&mut self, state: MazeState, score: usize) {
        self.0[state.position[0]][state.position[1]][usize::from(state.direction)] = score;
    }
}

enum Task {
    LowestScore,
    CountTiles,
}

fn solution(input: String, task: Task) -> crate::PuzzleResult {
    let maze: Maze = input.parse()?;
    let mut scores = Scores(vec![vec![[usize::MAX; 4]; maze.width]; maze.height]);
    let mut to_visit = std::collections::HashSet::new();
    let mut predecessors = std::collections::HashMap::<MazeState, Vec<MazeState>>::new();
    let mut lowest_score = None;
    let start = maze.initial_state();
    scores.set(start, 0);
    to_visit.insert(start);
    'dijkstra: loop {
        let state = to_visit
            .iter()
            .min_by_key(|&&state| scores.get(state))
            .copied()
            .unwrap();
        to_visit.remove(&state);
        let score = scores.get(state);

        for (neighbor, transition_score) in maze.neighbor_states(state) {
            let neighbor_score = score + transition_score;

            if let Some(lowest_score) = lowest_score {
                if neighbor_score > lowest_score {
                    break 'dijkstra;
                }
            } else if neighbor.position == maze.end {
                if let Task::LowestScore = task {
                    return Ok(neighbor_score.to_string());
                }
                lowest_score = Some(neighbor_score);
            }

            let old_neighbor_score = scores.get(neighbor);
            if neighbor_score <= old_neighbor_score {
                if old_neighbor_score == usize::MAX {
                    to_visit.insert(neighbor);
                }
                if neighbor_score == old_neighbor_score {
                    predecessors.get_mut(&neighbor).unwrap().push(state);
                } else {
                    predecessors.insert(neighbor, vec![state]);
                }
                scores.set(neighbor, neighbor_score);
            }
        }
    }

    let mut lowest_score_path_tiles = std::collections::HashSet::new();
    let mut to_visit = Vec::new();
    for direction in [East, North, West, South] {
        to_visit.push(MazeState {
            position: maze.end,
            direction,
        });
    }
    while !to_visit.is_empty() {
        let state = to_visit.pop().unwrap();
        lowest_score_path_tiles.insert(state.position);
        if let Some(predecessors) = predecessors.get(&state) {
            for &predecessor in predecessors {
                to_visit.push(predecessor);
            }
        }
    }

    Ok(lowest_score_path_tiles.len().to_string())
}

/// Part 1: Lowest possible score
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Task::LowestScore)
}

/// Part 2: Number of tiles part of a lowest score path
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, Task::CountTiles)
}

#[cfg(test)]
mod tests {
    const FIRST: &str = concat!(
        "###############\n",
        "#.......#....E#\n",
        "#.#.###.#.###.#\n",
        "#.....#.#...#.#\n",
        "#.###.#####.#.#\n",
        "#.#.#.......#.#\n",
        "#.#.#####.###.#\n",
        "#...........#.#\n",
        "###.#.#####.#.#\n",
        "#...#.....#.#.#\n",
        "#.#.#.###.#.#.#\n",
        "#.....#...#.#.#\n",
        "#.###.#.#.#.#.#\n",
        "#S..#.....#...#\n",
        "###############",
    );

    const SECOND: &str = concat!(
        "#################\n",
        "#...#...#...#..E#\n",
        "#.#.#.#.#.#.#.#.#\n",
        "#.#.#.#...#...#.#\n",
        "#.#.#.#.###.#.#.#\n",
        "#...#.#.#.....#.#\n",
        "#.#.#.#.#.#####.#\n",
        "#.#...#.#.#.....#\n",
        "#.#.#####.#.###.#\n",
        "#.#.#.......#...#\n",
        "#.#.###.#####.###\n",
        "#.#.#...#.....#.#\n",
        "#.#.#.#####.###.#\n",
        "#.#.#.........#.#\n",
        "#.#.#.#########.#\n",
        "#S#.............#\n",
        "#################",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(FIRST.to_string()).unwrap(), "7036");
        assert_eq!(&super::part1(SECOND.to_string()).unwrap(), "11048");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(FIRST.to_string()).unwrap(), "45");
        assert_eq!(&super::part2(SECOND.to_string()).unwrap(), "64");
    }
}
