use advent_of_code::{current_year, download_input};
use clap::Parser;

const DEFAULT_TOKEN_PATH: &str = "token.txt";
const DEFAULT_INPUT_PATH: &str = "input";

#[derive(Parser)]
struct Args {
    /// Between 1 and 25
    day: usize,

    /// 1 or 2
    part: usize,

    #[arg(short, long, default_value_t = current_year())]
    year: usize,

    /// Path to directory with input files {input}/{year}/{day:02}.txt
    #[arg(short, long, default_value_t = DEFAULT_INPUT_PATH.to_owned())]
    input: String,

    /// Path to session token file for downloading input if not found
    #[arg(short, long, default_value_t = DEFAULT_TOKEN_PATH.to_owned())]
    token: String,

    /// Wether to download and overwrite an existing input file
    #[arg(short, long)]
    overwrite: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    if ![1, 2].contains(&args.part) {
        Err(format!("part must be 1 or 2 not {}", args.part))?
    }
    let path = format!("{}/{}/{:02}.txt", args.input, args.year, args.day);

    // Read or download puzzle input
    let input = if !args.overwrite && std::path::Path::new(&path).exists() {
        println!("Reading input from {path}");
        std::fs::read_to_string(&path).or(Err(format!("couldn't read {path}")))
    } else {
        println!("Downloading input into {path}");
        std::fs::read_to_string(&args.token)
            .or(Err(format!("couldn't read token from {}", args.token)))
            .and_then(|token| download_input(token, args.year, args.day))
            .and_then(|input| {
                // Try to write puzzle input
                if std::path::Path::new(&path)
                    .parent()
                    .is_some_and(|parent| std::fs::create_dir_all(parent).is_err())
                {
                    eprintln!("Warning: \"couldn't make directories for {path}\"");
                } else {
                    use std::io::Write;
                    if !std::fs::File::create(&path)
                        .is_ok_and(|mut buffer| buffer.write_all(input.as_bytes()).is_ok())
                    {
                        eprintln!("Warning: \"couldn't write input to {path}\"");
                    }
                }
                Ok(input)
            })
    }?;

    // Solve puzzle
    match (args.year, args.day, args.part) {
        (year, day, part) => Err(format!(
            "no implementation for day {day} part {part} of {year}"
        )),
    }?
}
