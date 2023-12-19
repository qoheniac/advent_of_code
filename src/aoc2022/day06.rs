//! # Day 6: Tuning Trouble
//!
//! [puzzle site](https://adventofcode.com/2022/day/6)

fn solution(input: String, marker_size: usize) -> crate::PuzzleResult {
    let mut characters = input.chars();
    let mut last = std::collections::VecDeque::new();
    for _ in 0..marker_size {
        last.push_back(characters.next().unwrap());
    }
    let mut count = marker_size;
    loop {
        let mut is_start_of_packet = true;
        'sop_detection: for i in 0..marker_size {
            for j in (i + 1)..marker_size {
                if last[i] == last[j] {
                    is_start_of_packet = false;
                    count += 1;
                    break 'sop_detection;
                }
            }
        }
        if is_start_of_packet {
            break;
        }
        if let Some(c) = characters.next() {
            last.pop_front();
            last.push_back(c);
        } else {
            break;
        }
    }
    Ok(count.to_string())
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, 4)
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, 14)
}

#[cfg(test)]
mod tests {
    const INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_part1() {
        const RESULTS: [&str; INPUTS.len()] = ["7", "5", "6", "10", "11"];
        for i in 0..INPUTS.len() {
            assert_eq!(&super::part1(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }

    #[test]
    fn test_part2() {
        const RESULTS: [&str; INPUTS.len()] = ["19", "23", "23", "29", "26"];
        for i in 0..INPUTS.len() {
            assert_eq!(&super::part2(INPUTS[i].to_string()).unwrap(), RESULTS[i]);
        }
    }
}
