//! # Day 12: Garden Groups
//!
//! The input consists of an rectangular matrix of characters representing
//! garden plots with a certain plant. Neighboring garden plots with the same
//! plant are grouped together and get a fence which costs a price that equals
//! the product of the group's area and perimeter.
//!
//! [puzzle site](https://adventofcode.com/2024/day12)

/// Part 1: Total price for all fences
pub fn part1(input: String) -> crate::PuzzleResult {
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
    let mut price = 0;
    for region in regions {
        let mut perimeter = 0;
        for &[i, j] in &region {
            perimeter += !region.contains(&[i, j + 1]) as usize
                + (i == 0 || !region.contains(&[i - 1, j])) as usize
                + (j == 0 || !region.contains(&[i, j - 1])) as usize
                + !region.contains(&[i + 1, j]) as usize;
        }
        price += perimeter * region.len();
    }
    Ok(price.to_string())
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

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(FIRST.to_string()).unwrap(), "140");
        assert_eq!(&super::part1(SECOND.to_string()).unwrap(), "772");
        assert_eq!(&super::part1(THIRD.to_string()).unwrap(), "1930");
    }
}
