//! # Day 8: Playground
//!
//! The input holds one 3D location per line defined by three comma-separated
//! coordinates.
//!
//! [puzzle site](https://adventofcode.com/2025/day/8)

fn solution(input: String, subset: Option<usize>) -> crate::PuzzleResult {
    // parse input
    let mut locations: Vec<[u64; 3]> = Vec::new();
    for line in input.lines() {
        let mut coordinates = line.split(",");
        let x = coordinates.next().ok_or("x missing")?.parse()?;
        let y = coordinates.next().ok_or("y missing")?.parse()?;
        let z = coordinates.next().ok_or("z missing")?.parse()?;
        locations.push([x, y, z]);
    }
    let count = locations.len();

    // find n shortest connections (if subset is None sort all connections)
    let n = subset.unwrap_or(count * (count - 1) / 2);
    let mut connections = Vec::new();
    for (i, r_i) in locations.iter().enumerate().take(count - 1) {
        for (j, r_j) in locations.iter().enumerate().skip(i + 1) {
            let d: u64 = (r_i.iter().zip(r_j).map(|p| p.0.abs_diff(*p.1).pow(2))).sum();
            let mut insertion_index = connections.len();
            for (k, (_, distance)) in connections.iter().enumerate() {
                if d < *distance {
                    insertion_index = k;
                    break;
                }
            }
            if insertion_index < n {
                connections.insert(insertion_index, ([i, j], d));
            }
            if connections.len() > n {
                connections.pop();
            }
        }
    }

    // cluster locations with respect to connections
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    for ([i, j], _) in connections {
        let mut i_cluster_index = None;
        for (k, cluster) in clusters.iter().enumerate() {
            if cluster.contains(&i) {
                i_cluster_index = Some(k);
                break;
            }
        }
        let mut j_cluster_index = None;
        for (k, cluster) in clusters.iter().enumerate() {
            if cluster.contains(&j) {
                j_cluster_index = Some(k);
                break;
            }
        }
        match (i_cluster_index, j_cluster_index) {
            (None, None) => clusters.push(vec![i, j]),
            (Some(k), None) => clusters[k].push(j),
            (None, Some(k)) => clusters[k].push(i),
            (Some(k_i), Some(k_j)) => {
                if k_i != k_j {
                    let k_min = k_i.min(k_j);
                    let k_max = k_i.max(k_j);
                    let mut cluster = clusters.remove(k_max);
                    clusters[k_min].append(&mut cluster);
                }
            }
        }

        // Part 2 solution
        if subset.is_none() && clusters[0].len() == count {
            return Ok((locations[i][0] * locations[j][0]).to_string());
        }
    }

    // Part 1 solution
    let mut lengths: Vec<_> = clusters.iter().map(|c| c.len()).collect();
    lengths.sort_unstable();
    Ok((lengths.iter().rev().take(3).fold(1, |acc, len| acc * len)).to_string())
}

/// Part 1: Product of the number of locations in the three largest clusters
/// formed by connecting the 1000 locations with the shortest distance
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Some(1000))
}

/// Part 2: Product of the x-components of the last location pair, when pairs
/// are considered in ascending distance order and connected until all locations
/// form one cluster
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, None)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "162,817,812\n",
        "57,618,57\n",
        "906,360,560\n",
        "592,479,940\n",
        "352,342,300\n",
        "466,668,158\n",
        "542,29,236\n",
        "431,825,988\n",
        "739,650,466\n",
        "52,470,668\n",
        "216,146,977\n",
        "819,987,18\n",
        "117,168,530\n",
        "805,96,715\n",
        "346,949,466\n",
        "970,615,88\n",
        "941,993,340\n",
        "862,61,35\n",
        "984,92,344\n",
        "425,690,689",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::solution(INPUT.to_string(), Some(10)).unwrap(), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "25272");
    }
}
