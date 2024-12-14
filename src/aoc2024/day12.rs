//! # Day 12: Garden Groups
//!
//! The input consists of an rectangular matrix of characters representing
//! garden plots with a certain plant. Neighboring garden plots with the same
//! plant are grouped together and get a fence which costs a price that equals
//! the product of the group's area and perimeter. With bulk discount the number
//! of sides instead of the perimeter is used to calculate the price. The goal
//! is to calculate the total price to buy fences for all regions.
//!
//! [puzzle site](https://adventofcode.com/2024/day12)

fn regions(input: String) -> Vec<Vec<[usize; 2]>> {
    let mut plants: Vec<Vec<(char, usize)>> = Vec::new();
    let mut regions: Vec<Vec<[usize; 2]>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, plant) in line.chars().enumerate() {
            let region = if let Some(region) = if i > 0 && plants[i - 1][j].0 == plant {
                // belongs to region above
                let mut region = plants[i - 1][j].1;
                if j > 0 && plants[i][j - 1].0 == plant && plants[i][j - 1].1 != region {
                    // also belongs to region to the left -> merge regions
                    let mut other_region = plants[i][j - 1].1;
                    (region, other_region) = (region.min(other_region), region.max(other_region));
                    let mut other_region_plants = regions.remove(other_region);
                    for &[i, j] in &other_region_plants {
                        plants[i][j] = (plant, region);
                    }
                    for higher_region_plants in &regions[other_region..] {
                        for &[i, j] in higher_region_plants {
                            plants[i][j] = (plants[i][j].0, plants[i][j].1 - 1);
                        }
                    }
                    regions[region].append(&mut other_region_plants);
                }
                Some(region)
            } else if j > 0 && plants[i][j - 1].0 == plant {
                // belongs to region to the left
                Some(plants[i][j - 1].1)
            } else {
                None
            } {
                // add plant location to known region
                regions[region].push([i, j]);
                region
            } else {
                // add plant location to new region
                regions.push(vec![[i, j]]);
                regions.len() - 1
            };
            // add plant to plants
            if j == 0 {
                plants.push(vec![(plant, region)]);
            } else {
                plants[i].push((plant, region));
            }
        }
    }
    regions
}

fn price_without_discount(region: Vec<[usize; 2]>) -> usize {
    let mut perimeter = 0;
    for &[i, j] in &region {
        perimeter += !region.contains(&[i, j + 1]) as usize
            + (i == 0 || !region.contains(&[i - 1, j])) as usize
            + (j == 0 || !region.contains(&[i, j - 1])) as usize
            + !region.contains(&[i + 1, j]) as usize;
    }
    perimeter * region.len()
}

fn price_with_discount(region: Vec<[usize; 2]>) -> usize {
    let mut east = Vec::new();
    let mut north = Vec::new();
    let mut west = Vec::new();
    let mut south = Vec::new();
    let mut sides = 0;
    for &[i, j] in &region {
        if !region.contains(&[i, j + 1]) {
            east.push([i, j]);
            if (i == 0 || !east.contains(&[i - 1, j])) && !east.contains(&[i + 1, j]) {
                sides += 1;
            }
        }
        if i == 0 || !region.contains(&[i - 1, j]) {
            north.push([i, j]);
            if !north.contains(&[i, j + 1]) && (j == 0 || !north.contains(&[i, j - 1])) {
                sides += 1;
            }
        }
        if j == 0 || !region.contains(&[i, j - 1]) {
            west.push([i, j]);
            if (i == 0 || !west.contains(&[i - 1, j])) && !west.contains(&[i + 1, j]) {
                sides += 1;
            }
        }
        if !region.contains(&[i + 1, j]) {
            south.push([i, j]);
            if !south.contains(&[i, j + 1]) && (j == 0 || !south.contains(&[i, j - 1])) {
                sides += 1;
            }
        }
    }
    sides * region.len()
}

fn price(regions: Vec<Vec<[usize; 2]>>, with_discount: bool) -> usize {
    regions
        .into_iter()
        .map(if with_discount {
            price_with_discount
        } else {
            price_without_discount
        })
        .sum()
}

/// Part 1: Without bulk discount
pub fn part1(input: String) -> crate::PuzzleResult {
    Ok(price(regions(input), false).to_string())
}

/// Part 2: With bulk discount
pub fn part2(input: String) -> crate::PuzzleResult {
    Ok(price(regions(input), true).to_string())
}

#[cfg(test)]
mod tests {
    const FIRST: &str = "AAAA\nBBCD\nBBCC\nEEEC";
    const SECOND: &str = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
    const THIRD: &str = concat!(
        "RRRRIICCFF\n",
        "RRRRIICCCF\n",
        "VVRRRCCFFF\n",
        "VVRCCCJFFF\n",
        "VVVVCJJCFE\n",
        "VVIVCCJJEE\n",
        "VVIIICJJEE\n",
        "MIIIIIJJEE\n",
        "MIIISIJEEE\n",
        "MMMISSJEEE",
    );
    const FORTH: &str = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
    const FIFTH: &str = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(FIRST.to_string()).unwrap(), "140");
        assert_eq!(&super::part1(SECOND.to_string()).unwrap(), "772");
        assert_eq!(&super::part1(THIRD.to_string()).unwrap(), "1930");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(FIRST.to_string()).unwrap(), "80");
        assert_eq!(&super::part2(SECOND.to_string()).unwrap(), "436");
        assert_eq!(&super::part2(THIRD.to_string()).unwrap(), "1206");
        assert_eq!(&super::part2(FORTH.to_string()).unwrap(), "236");
        assert_eq!(&super::part2(FIFTH.to_string()).unwrap(), "368");
    }
}
