//! # Day 12: Hill Climbing Algorithm
//!
//! [puzzle site](https://adventofcode.com/2022/day/12)

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, i: usize, j: usize) -> char {
        self.0[i][j]
    }

    fn elevation(&self, i: usize, j: usize) -> u32 {
        match self.get(i, j) {
            'S' => 0,
            'E' => 25,
            other => other.to_digit(36).unwrap() - 10,
        }
    }

    fn dijkstra(&self, i_start: usize, j_start: usize) -> usize {
        // initialize Dijkstra algorithm
        let mut distance = vec![vec![usize::MAX; self.width()]; self.height()];
        let mut to_visit = std::collections::HashSet::new();
        distance[i_start][j_start] = 0;
        to_visit.insert([i_start, j_start]);

        'dijkstra: loop {
            if to_visit.is_empty() {
                break usize::MAX;
            }

            // visit location with shortest path to get there
            let p_short = to_visit
                .iter()
                .min_by_key(|p| distance[p[0]][p[1]])
                .cloned()
                .unwrap();
            to_visit.remove(&p_short);
            let [i_short, j_short] = p_short;
            let e_short = self.elevation(i_short, j_short);

            // loop over reachable destinations
            for (m, n) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let i = (i_short as i32 + m) as usize; // -1 becomes usize::MAX and thus …
                let j = (j_short as i32 + n) as usize; // … larger than width or height
                if i < self.height() && j < self.width() && self.elevation(i, j) <= e_short + 1 {
                    let d = distance[i_short][j_short] + 1;

                    // final destination reached
                    if self.get(i, j) == 'E' {
                        break 'dijkstra d;
                    }

                    // update distance
                    if distance[i][j] == usize::MAX {
                        distance[i][j] = d;
                        to_visit.insert([i, j]);
                    } else if d < distance[i][j] {
                        distance[i][j] = d;
                    }
                }
            }
        }
    }
}

impl From<String> for Grid {
    fn from(string: String) -> Self {
        Self(string.lines().map(|line| line.chars().collect()).collect())
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let grid = Grid::from(input);
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.get(i, j) == 'S' {
                return Ok(grid.dijkstra(i, j).to_string());
            }
        }
    }
    Err("start not found")?
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let grid = Grid::from(input);
    let mut shortest = usize::MAX;
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.get(i, j) == 'a' {
                shortest = shortest.min(grid.dijkstra(i, j));
            }
        }
    }
    Ok(shortest.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "Sabqponm\n",
        "abcryxxl\n",
        "accszExk\n",
        "acctuvwj\n",
        "abdefghi",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "31");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "29");
    }
}
