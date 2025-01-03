//! # Advent of Code Solutions
//!
//! [puzzle site](https://adventofcode.com)

#![warn(missing_docs)]

use chrono::{Datelike, Utc};
use reqwest::blocking::Client;

/// # Solutions for 2022
///
/// [puzzle site](https://adventofcode.com/2022)
pub mod aoc2022 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
}

/// # Solutions for 2023
///
/// [puzzle site](https://adventofcode.com/2023)
pub mod aoc2023 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
}

/// # Solutions for 2024
///
/// [puzzle site](https://adventofcode.com/2024)
pub mod aoc2024 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
}

/// Common return type of puzzle solutions
pub type PuzzleResult = Result<String, Box<dyn std::error::Error>>;

/// Returns the input for a certain Advent of Code puzzle or an error if the download fails
pub fn download_input(session_token: String, year: usize, day: usize) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    Client::new()
        .get(&url)
        .header("Cookie", format!("session={session_token}"))
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())
        .or(Err(format!("couldn't download from {url}")))
}

/// Returns the year of the most recent Advent of Code
pub fn current_year() -> usize {
    let date = Utc::now();
    let year = date.year();
    (if date.month() == 12 { year } else { year - 1 }) as usize
}

/// Solve a certain Advent of Code puzzle
pub fn solve(year: usize, day: usize, part: usize, input: String) -> PuzzleResult {
    match (year, day, part) {
        (2022, 1, 1) => aoc2022::day01::part1(input),
        (2022, 1, 2) => aoc2022::day01::part2(input),
        (2022, 2, 1) => aoc2022::day02::part1(input),
        (2022, 2, 2) => aoc2022::day02::part2(input),
        (2022, 3, 1) => aoc2022::day03::part1(input),
        (2022, 3, 2) => aoc2022::day03::part2(input),
        (2022, 4, 1) => aoc2022::day04::part1(input),
        (2022, 4, 2) => aoc2022::day04::part2(input),
        (2022, 5, 1) => aoc2022::day05::part1(input),
        (2022, 5, 2) => aoc2022::day05::part2(input),
        (2022, 6, 1) => aoc2022::day06::part1(input),
        (2022, 6, 2) => aoc2022::day06::part2(input),
        (2022, 7, 1) => aoc2022::day07::part1(input),
        (2022, 7, 2) => aoc2022::day07::part2(input),
        (2022, 8, 1) => aoc2022::day08::part1(input),
        (2022, 8, 2) => aoc2022::day08::part2(input),
        (2022, 9, 1) => aoc2022::day09::part1(input),
        (2022, 9, 2) => aoc2022::day09::part2(input),
        (2022, 10, 1) => aoc2022::day10::part1(input),
        (2022, 10, 2) => aoc2022::day10::part2(input),
        (2022, 11, 1) => aoc2022::day11::part1(input),
        (2022, 11, 2) => aoc2022::day11::part2(input),
        (2022, 12, 1) => aoc2022::day12::part1(input),
        (2022, 12, 2) => aoc2022::day12::part2(input),
        (2022, 13, 1) => aoc2022::day13::part1(input),
        (2022, 13, 2) => aoc2022::day13::part2(input),

        (2023, 1, 1) => aoc2023::day01::part1(input),
        (2023, 1, 2) => aoc2023::day01::part2(input),
        (2023, 2, 1) => aoc2023::day02::part1(input),
        (2023, 2, 2) => aoc2023::day02::part2(input),
        (2023, 3, 1) => aoc2023::day03::part1(input),
        (2023, 3, 2) => aoc2023::day03::part2(input),
        (2023, 4, 1) => aoc2023::day04::part1(input),
        (2023, 4, 2) => aoc2023::day04::part2(input),
        (2023, 5, 1) => aoc2023::day05::part1(input),
        (2023, 5, 2) => aoc2023::day05::part2(input),
        (2023, 6, 1) => aoc2023::day06::part1(input),
        (2023, 6, 2) => aoc2023::day06::part2(input),
        (2023, 7, 1) => aoc2023::day07::part1(input),
        (2023, 7, 2) => aoc2023::day07::part2(input),
        (2023, 8, 1) => aoc2023::day08::part1(input),
        (2023, 8, 2) => aoc2023::day08::part2(input),
        (2023, 9, 1) => aoc2023::day09::part1(input),
        (2023, 9, 2) => aoc2023::day09::part2(input),
        (2023, 10, 1) => aoc2023::day10::part1(input),
        (2023, 10, 2) => aoc2023::day10::part2(input),
        (2023, 11, 1) => aoc2023::day11::part1(input),
        (2023, 11, 2) => aoc2023::day11::part2(input),
        (2023, 12, 1) => aoc2023::day12::part1(input),
        (2023, 12, 2) => aoc2023::day12::part2(input),
        (2023, 13, 1) => aoc2023::day13::part1(input),
        (2023, 13, 2) => aoc2023::day13::part2(input),
        (2023, 14, 1) => aoc2023::day14::part1(input),
        (2023, 14, 2) => aoc2023::day14::part2(input),
        (2023, 15, 1) => aoc2023::day15::part1(input),
        (2023, 15, 2) => aoc2023::day15::part2(input),
        (2023, 16, 1) => aoc2023::day16::part1(input),
        (2023, 16, 2) => aoc2023::day16::part2(input),
        (2023, 17, 1) => aoc2023::day17::part1(input),
        (2023, 17, 2) => aoc2023::day17::part2(input),
        (2023, 18, 1) => aoc2023::day18::part1(input),
        (2023, 18, 2) => aoc2023::day18::part2(input),

        (2024, 1, 1) => aoc2024::day01::part1(input),
        (2024, 1, 2) => aoc2024::day01::part2(input),
        (2024, 2, 1) => aoc2024::day02::part1(input),
        (2024, 2, 2) => aoc2024::day02::part2(input),
        (2024, 3, 1) => aoc2024::day03::part1(input),
        (2024, 3, 2) => aoc2024::day03::part2(input),
        (2024, 4, 1) => aoc2024::day04::part1(input),
        (2024, 4, 2) => aoc2024::day04::part2(input),
        (2024, 5, 1) => aoc2024::day05::part1(input),
        (2024, 5, 2) => aoc2024::day05::part2(input),
        (2024, 6, 1) => aoc2024::day06::part1(input),
        (2024, 6, 2) => aoc2024::day06::part2(input),
        (2024, 7, 1) => aoc2024::day07::part1(input),
        (2024, 7, 2) => aoc2024::day07::part2(input),
        (2024, 8, 1) => aoc2024::day08::part1(input),
        (2024, 8, 2) => aoc2024::day08::part2(input),
        (2024, 9, 1) => aoc2024::day09::part1(input),
        (2024, 9, 2) => aoc2024::day09::part2(input),
        (2024, 10, 1) => aoc2024::day10::part1(input),
        (2024, 10, 2) => aoc2024::day10::part2(input),
        (2024, 11, 1) => aoc2024::day11::part1(input),
        (2024, 11, 2) => aoc2024::day11::part2(input),
        (2024, 12, 1) => aoc2024::day12::part1(input),
        (2024, 12, 2) => aoc2024::day12::part2(input),
        (2024, 13, 1) => aoc2024::day13::part1(input),
        (2024, 13, 2) => aoc2024::day13::part2(input),
        (2024, 14, 1) => aoc2024::day14::part1(input),
        (2024, 14, 2) => aoc2024::day14::part2(input),
        (2024, 15, 1) => aoc2024::day15::part1(input),
        (2024, 15, 2) => aoc2024::day15::part2(input),
        (2024, 16, 1) => aoc2024::day16::part1(input),
        (2024, 16, 2) => aoc2024::day16::part2(input),
        (2024, 17, 1) => aoc2024::day17::part1(input),

        (year, day, part) => Err(format!("no solution for day {day} part {part} of {year}"))?,
    }
}
