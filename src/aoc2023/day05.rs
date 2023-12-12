//! Day 5: If You Give A Seed A Fertilizer
//!
//! The first line lists some seed numbers and the blocks of lines after that
//! describe mappings consisting of a set of ranges that map numbers by a
//! certain distance. The ranges are described by three numbers where the first
//! one is the start of the destination range, the second one is the start of
//! the source range, and the last one is the length of the ranges. If no range
//! applies, numbers stay unchanged. Successively applying all maps to the seed
//! numbers gives the respective location numbers

// Range of numbers mapping some distance in positive or negative direction
struct MapRange {
    range: std::ops::Range<usize>,
    distance: usize,
    is_subtracting: bool,
}

// Mapping a number by trying a number of MapRanges or map to itself
fn destination(map: &Vec<MapRange>, source: usize) -> usize {
    for map_range in map {
        if map_range.range.contains(&source) {
            return if map_range.is_subtracting {
                source - map_range.distance
            } else {
                source + map_range.distance
            };
        }
    }
    source
}

// Return the next number from a string of whitespace-separated numbers
fn next_number(numbers: &mut std::str::SplitWhitespace) -> Option<usize> {
    numbers.next().and_then(|number| number.parse().ok())
}

/// Part 1: Lowest location number
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut blocks = input.split("\n\n");

    // Parse seeds
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

    // Parse maps
    let mut maps = Vec::new();
    for block in blocks {
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

    // Apply maps and find lowest location
    let mut location = usize::MAX;
    for seed in seeds {
        let mut number = seed;
        for map in &maps {
            number = destination(map, number)
        }
        location = location.min(number);
    }
    Ok(location.to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input: String = "\
            seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4"
            .to_string();
        assert_eq!(&super::part1(input).unwrap(), "35");
    }
}
