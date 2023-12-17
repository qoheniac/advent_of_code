//! # Day 6: Wait For It
//!
//! The first line has a list of integer times and the second line a list of
//! integer distances. Setting up the velocity takes one time unit for each
//! velocity unit, that is one distance unit per time unit. Each time value
//! \\(t\\) gives a total amount of time available and each distance value
//! \\(d\\) a minimum distance that should be reached. The velocity \\(v\\)
//! necessary to reach exactly the distance \\(d\\) is
//!
//! \\[ d = v (t - v) \\quad \\Rightarrow \\quad
//! v_\\pm = \\tfrac{1}{2} t \\pm \\sqrt{\\tfrac{1}{4} t^2 - d} \\]
//!
//! Every velocity between these two velocity solutions will result in a
//! distance larger than \\(d\\). Because only integer velocities are allowed,
//! the smallest velocity to reach further than \\(d\\) is the next integer
//! larger than \\(v_-\\) and the largest one is the next integer smaller than
//! \\(v_+\\).
//!
//! [puzzle site](https://adventofcode.com/2023/day/6)

fn ways((time, distance): (f64, f64)) -> usize {
    let sqrt = (time * time / 4.0 - distance).sqrt();
    let min = (time / 2.0 - sqrt).floor() as usize + 1;
    let max = (time / 2.0 + sqrt).ceil() as usize - 1;
    1 + max - min
}

/// Part 1: Product of ways to reach further than the given distance in the
/// given time over all input columns
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|number| number.parse::<f64>().ok())
            .flatten()
    });
    let times = lines.next().ok_or("times not found")?;
    let distances = lines.next().ok_or("distances not found")?;
    let product: usize = times.zip(distances).map(ways).product();
    Ok(product.to_string())
}

/// Part 2: Ways to reach further than the given distance in the given time
/// where numbers are found by ignoring whitespace
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<f64>()
                .ok()
        })
        .flatten();
    let time = lines.next().ok_or("time not found")?;
    let distance = lines.next().ok_or("distance not found")?;
    let ways = ways((time, distance));
    Ok(ways.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "288");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "71503");
    }
}
