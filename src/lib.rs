//! [Advent of Code](https://adventofcode.com/) solutions

#![warn(missing_docs)]

use chrono::{Datelike, Utc};
use reqwest::blocking::Client;

/// Solutions for 2023
pub mod aoc2023 {
    pub mod day01;
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
        .and_then(|response| response.text())
        .or(Err(format!("couldn't download from {url}")))
}

/// Returns the year of the most recent Advent of Code
pub fn current_year() -> usize {
    let date = Utc::now();
    let year = date.year();
    (if date.month() == 12 { year } else { year - 1 }) as usize
}
