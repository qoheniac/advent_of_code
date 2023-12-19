//! # Day 8: Haunted Wasteland
//!
//! The first line of the input gives a list of instructions L or R. After that
//! a list of node mappings follows that map each node to two other nodes.
//! Following the instructions means to change from one node to either the first
//! node (L) in its mapping or the second node (R).
//!
//! [puzzle site](https://adventofcode.com/2023/day/8)

use num::Integer; // lowest common multiple
use regex::Regex;

type Instructions<'a> = std::iter::Cycle<std::str::Chars<'a>>;
type Nodes<'a> = std::collections::HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input<'a>(input: &'a str) -> Result<(Instructions<'a>, Nodes<'a>), String> {
    let blocks = input.split_once("\n\n").ok_or("no empty line found")?;
    let instructions = blocks.0.chars().cycle();
    let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
    let mut nodes = Nodes::new();
    for (_, [node, left, right]) in re.captures_iter(blocks.1).map(|c| c.extract()) {
        nodes.insert(node, (left, right));
    }
    Ok((instructions, nodes))
}

fn follow_instruction<'a>(
    nodes: &'a Nodes,
    location: &str,
    instruction: char,
) -> Result<&'a str, String> {
    let targets = nodes.get(location).ok_or(format!("{location} not found"))?;
    match instruction {
        'L' => Ok(targets.0),
        'R' => Ok(targets.1),
        instruction => Err(format!("{instruction} is not a valid instruction")),
    }
}

/// Part 1: Going from AAA to ZZZ
pub fn part1(input: String) -> crate::PuzzleResult {
    let (mut instructions, nodes) = parse_input(&input)?;
    let mut steps = 0;
    let mut location = "AAA";
    while location != "ZZZ" {
        steps += 1;
        location = follow_instruction(&nodes, location, instructions.next().unwrap())?;
    }
    Ok(steps.to_string())
}

/// Part 2: Simultaneously from all ..A to only ..Z
pub fn part2(input: String) -> crate::PuzzleResult {
    let (mut instructions, nodes) = parse_input(&input)?;

    // Collect start locations
    let mut locations = std::collections::HashSet::new();
    for &node in nodes.keys() {
        if node.chars().last().unwrap() == 'A' {
            locations.insert(node);
        }
    }

    // Iterate while not all paths reached their destination
    let mut steps: usize = 0;
    let mut lowest_common_multiple = 1;
    while !locations.is_empty() {
        steps += 1;

        // Follow instructions
        let instruction = instructions.next().unwrap();
        for location in &locations.clone() {
            locations.remove(location);
            locations.insert(follow_instruction(&nodes, location, instruction)?);
        }

        // Check if destination is reached
        for location in &locations.clone() {
            if location.chars().last().unwrap() == 'Z' {
                locations.remove(location);
                lowest_common_multiple = lowest_common_multiple.lcm(&steps);
            }
        }
    }
    Ok(lowest_common_multiple.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        const INPUTS: [&str; 2] = [
            concat!(
                "RL\n",
                "\n",
                "AAA = (BBB, CCC)\n",
                "BBB = (DDD, EEE)\n",
                "CCC = (ZZZ, GGG)\n",
                "DDD = (DDD, DDD)\n",
                "EEE = (EEE, EEE)\n",
                "GGG = (GGG, GGG)\n",
                "ZZZ = (ZZZ, ZZZ)"
            ),
            concat!(
                "LLR\n",
                "\n",
                "AAA = (BBB, BBB)\n",
                "BBB = (AAA, ZZZ)\n",
                "ZZZ = (ZZZ, ZZZ)"
            ),
        ];
        const RESULTS: [&str; INPUTS.len()] = ["2", "6"];
        for i in 0..INPUTS.len() {
            assert_eq!(&super::part1(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)"
        );
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "6");
    }
}
