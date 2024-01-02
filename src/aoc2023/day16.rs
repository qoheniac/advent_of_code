//! # Day 16: The Floor Will Be Lava
//!
//! The input contains a map of an optical contraption where dots mark empty
//! tiles, slashes mark diagonal mirrors and bars mark beam splitters that
//! parallel beams will pass and orthogonal beams will split in the two
//! directions parallel to the splitter. Tiles traversed by a beam are called
//! energized.
//!
//! [puzzle site](https://adventofcode.com/2023/day/16)

use std::usize;

use itertools::Itertools;
use ndarray::Array2;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location(i8, i8);

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
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct BeamSegment(Location, Direction);

#[derive(Clone, Copy)]
enum MirrorOrientation {
    PositiveSlope,
    NegativeSlope,
}
use MirrorOrientation::*;

impl MirrorOrientation {
    fn reflect(&self, direction: Direction) -> Direction {
        match (*self, direction) {
            (PositiveSlope, Down) | (NegativeSlope, Up) => Left,
            (PositiveSlope, Left) | (NegativeSlope, Right) => Down,
            (PositiveSlope, Right) | (NegativeSlope, Left) => Up,
            (PositiveSlope, Up) | (NegativeSlope, Down) => Right,
        }
    }
}

#[derive(Clone, Copy)]
enum SplitterOrientation {
    Horizontal,
    Vertical,
}
use SplitterOrientation::*;

impl SplitterOrientation {
    fn pass(&self, direction: Direction) -> Vec<Direction> {
        match (*self, direction) {
            (Horizontal, Down | Up) => vec![Left, Right],
            (Vertical, Left | Right) => vec![Down, Up],
            _ => vec![direction],
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Mirror(MirrorOrientation),
    Space,
    Splitter(SplitterOrientation),
}
use Tile::*;

struct Contraption(Array2<Tile>);

impl Contraption {
    fn field(&self, Location(i, j): Location) -> Option<&Tile> {
        self.0.get((i as usize, j as usize))
    }

    fn step(&self, location: Location, direction: Direction) -> Option<Location> {
        let location = direction.step(location);
        self.field(location).and(Some(location))
    }

    fn propagate(&self, BeamSegment(location, direction): BeamSegment) -> Vec<BeamSegment> {
        let mut directions = Vec::new();
        if let Some(field) = self.field(location) {
            match *field {
                Mirror(mirror) => directions.push(mirror.reflect(direction)),
                Space => directions.push(direction),
                Splitter(splitter) => {
                    for direction in splitter.pass(direction) {
                        directions.push(direction)
                    }
                }
            }
        }
        directions
            .iter()
            .flat_map(|&direction| {
                self.step(location, direction)
                    .and_then(|location| Some(BeamSegment(location, direction)))
            })
            .collect()
    }

    fn energized(&self, beam_start: BeamSegment) -> usize {
        let mut beam = std::collections::HashSet::new();
        let mut segments = vec![beam_start];
        while !segments.is_empty() {
            let mut next_segments = Vec::new();
            for segment in segments {
                if beam.insert(segment) {
                    next_segments.append(&mut self.propagate(segment));
                }
            }
            segments = next_segments;
        }
        beam.into_iter().unique_by(|s| s.0).count()
    }
}

impl std::str::FromStr for Contraption {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            height += 1;
            width = 0;
            for character in line.chars() {
                width += 1;
                fields.push(match character {
                    '/' => Mirror(PositiveSlope),
                    '\\' => Mirror(NegativeSlope),
                    '.' => Space,
                    '-' => Splitter(Horizontal),
                    '|' => Splitter(Vertical),
                    c => Err(format!("unexpected character {c}"))?,
                })
            }
        }
        Ok(Contraption(Array2::from_shape_vec(
            (height, width),
            fields,
        )?))
    }
}

/// Part 1: Tiles energized by a beam entering top left heading right
pub fn part1(input: String) -> crate::PuzzleResult {
    let contraption: Contraption = input.parse()?;
    let beam_start = BeamSegment(Location(0, 0), Right);
    Ok(contraption.energized(beam_start).to_string())
}

/// Part 2: Maximum tiles energized by a single incident beam
pub fn part2(input: String) -> crate::PuzzleResult {
    let contraption: Contraption = input.parse()?;
    let width = contraption.0.ncols();
    let height = contraption.0.nrows();
    let mut max = 0;
    for i in 0..height {
        for beam_start in [
            BeamSegment(Location(i as i8, 0), Right),
            BeamSegment(Location(i as i8, width as i8 - 1), Left),
        ] {
            max = max.max(contraption.energized(beam_start));
        }
    }
    for j in 0..width {
        for beam_start in [
            BeamSegment(Location(0, j as i8), Down),
            BeamSegment(Location(height as i8 - 1, j as i8), Up),
        ] {
            max = max.max(contraption.energized(beam_start));
        }
    }
    Ok(max.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "46");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "51");
    }
}
