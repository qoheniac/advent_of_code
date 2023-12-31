//! # Day 5: If You Give A Seed A Fertilizer
//!
//! The first line lists some seed numbers and the blocks of lines after that
//! describe mappings consisting of a set of ranges that map numbers by a
//! certain distance. The ranges are described by three numbers where the first
//! one is the start of the destination range, the second one is the start of
//! the source range, and the last one is the length of the ranges. If no range
//! applies, numbers stay unchanged. Successively applying all maps to the seed
//! numbers gives the respective location numbers where only the lowest one is
//! of interest.
//!
//! [puzzle site](https://adventofcode.com/2023/day/5)

// Range of numbers mapping some distance in positive or negative direction
struct MapRange {
    range: std::ops::Range<usize>,
    distance: usize,
    is_subtracting: bool,
}

// Return the next number from a string of whitespace-separated numbers
fn next_number(numbers: &mut std::str::SplitWhitespace) -> Option<usize> {
    numbers.next().and_then(|number| number.parse().ok())
}

// Parse maps text blocks into a maps vector where each map is MapRange vector
fn parse_maps(map_blocks: std::str::Split<&str>) -> Result<Vec<Vec<MapRange>>, String> {
    let mut maps = Vec::new();
    for block in map_blocks {
        let (name, block) = block.split_once(":\n").ok_or("couldn't parse block")?;
        let error = format!("couldn't parse {name}");
        let mut ranges = Vec::new();
        for line in block.lines() {
            let mut numbers = line.split_whitespace();
            let first_destination = next_number(&mut numbers).ok_or(error.clone())?;
            let first_source = next_number(&mut numbers).ok_or(error.clone())?;
            let range_length = next_number(&mut numbers).ok_or(error.clone())?;
            ranges.push(MapRange {
                range: first_source..(first_source + range_length),
                distance: first_destination.abs_diff(first_source),
                is_subtracting: first_destination < first_source,
            });
        }
        maps.push(ranges);
    }
    Ok(maps)
}

/// Part 1: First line lists seeds
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut blocks = input.split("\n\n");

    // Parse input
    let seeds: Vec<usize> = blocks
        .next()
        .and_then(|line| line.strip_prefix("seeds: "))
        .and_then(|numbers| {
            numbers
                .split_whitespace()
                .map(|number| number.parse().ok())
                .collect()
        })
        .ok_or("seeds not found")?;
    let maps = parse_maps(blocks)?;

    // Apply maps and find lowest location
    let mut location = usize::MAX;
    for seed in seeds {
        let mut number = seed;
        for map in &maps {
            for map_range in map {
                if map_range.range.contains(&number) {
                    if map_range.is_subtracting {
                        number -= map_range.distance;
                    } else {
                        number += map_range.distance;
                    };
                    break;
                }
            }
        }
        location = location.min(number);
    }
    Ok(location.to_string())
}

/// Part 2: First line lists seed ranges
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut blocks = input.split("\n\n");

    // Parse input
    let mut iter = blocks
        .next()
        .and_then(|line| line.strip_prefix("seeds: "))
        .ok_or("seeds not found")?
        .split_whitespace();
    let mut ranges = Vec::<(usize, usize)>::new();
    while let (Some(start), Some(length)) = (iter.next(), iter.next()) {
        if let (Ok(start), Ok(length)) = (start.parse(), length.parse()) {
            ranges.push((start, length));
        }
    }
    let maps = parse_maps(blocks)?;

    // Apply all maps to all seed ranges
    for map in &maps {
        let mut mapped_ranges = Vec::new();
        for (start, length) in ranges {
            let mut overlaps = Vec::new();
            for map_range in map {
                if map_range.range.start < start + length && start < map_range.range.end {
                    let overlap_start = start.max(map_range.range.start);
                    let overlap_length = (start + length).min(map_range.range.end) - overlap_start;
                    overlaps.push((overlap_start, overlap_length));

                    // Map overlap
                    let mapped_start = if map_range.is_subtracting {
                        overlap_start - map_range.distance
                    } else {
                        overlap_start + map_range.distance
                    };
                    mapped_ranges.push((mapped_start, overlap_length));
                }
            }

            // Map unmatched regions
            overlaps.sort();
            let mut unmapped_start = start;
            for (overlap_start, overlap_length) in overlaps {
                if unmapped_start < overlap_start {
                    mapped_ranges.push((unmapped_start, overlap_start - unmapped_start));
                }
                unmapped_start = overlap_start + overlap_length;
            }
            if unmapped_start < start + length {
                mapped_ranges.push((unmapped_start, start + length - unmapped_start));
            }
        }
        ranges = mapped_ranges;
    }

    // Find lowest location
    if let Some(min) = ranges.iter().map(|(start, _)| start).min() {
        Ok(min.to_string())
    } else {
        Err("no locations")?
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "35");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "46");
    }
}
