//! # Day 13: Claw Contraption
//!
//! The input consists of blocks representing claw machines separated by blank
//! lines. Each machine has two lines "Button [AB]: X\+[0-9]+, Y\+[0-9]+"
//! describing how the machine's two buttons move the claw and a third line
//! describing the target location "Price: X=[0-9]+, Y=[0-9]+". If the target
//! can be reached by pressing each button no more than 100 times, then the cost
//! to do that is 3 tokens each time the A button is pressed and 1 token each
//! time the B token is pressed. The goal is to calculate the number of tokens
//! required to get every reachable target. To correct for a unit conversion
//! error each component of the target location needs to be increased by
//! 10000000000000 and the limitation for how often buttons can be pressed is
//! lifted.
//!
//! [puzzle site](https://adventofcode.com/2024/day13)

fn parse_line(line: &str, symbol: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let error = format!("invalid line: {line}");
    let mut data = line
        .split(": ")
        .nth(1)
        .ok_or(error.clone())?
        .split(", ")
        .map(|expr| expr.split(symbol).nth(1).ok_or(error.clone()));
    let x = data.next().ok_or(error.clone())??.parse()?;
    let y = data.next().ok_or(error)??.parse()?;
    Ok((x, y))
}

fn solution(input: String, fix_unit_conversion_error: bool) -> crate::PuzzleResult {
    let mut price = 0;
    for machine in input.split("\n\n") {
        let mut lines = machine.lines();
        let error = format!("machine with too few lines:\n{machine}");
        let (dxa, dya) = parse_line(lines.next().ok_or(error.clone())?, "+")?;
        let (dxb, dyb) = parse_line(lines.next().ok_or(error.clone())?, "+")?;
        let (mut tx, mut ty) = parse_line(lines.next().ok_or(error.clone())?, "=")?;
        if fix_unit_conversion_error {
            tx += 10000000000000;
            ty += 10000000000000;
        }
        let denominator = dxb * dya - dxa * dyb;
        let numerator_a = dxb * ty - dyb * tx;
        let numerator_b = dya * tx - dxa * ty;
        if numerator_a % denominator == 0 && numerator_b % denominator == 0 {
            let a = numerator_a / denominator;
            let b = numerator_b / denominator;
            if (0..=100).contains(&a) && (0..=100).contains(&b)
                || fix_unit_conversion_error && a.is_positive() && b.is_positive()
            {
                price += 3 * a + b;
            }
        }
    }
    Ok(price.to_string())
}

/// Part 1: With unit conversion error
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, false)
}

/// Part 2: With unit conversion error
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, true)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "Button A: X+94, Y+34\n",
        "Button B: X+22, Y+67\n",
        "Prize: X=8400, Y=5400\n",
        "\n",
        "Button A: X+26, Y+66\n",
        "Button B: X+67, Y+21\n",
        "Prize: X=12748, Y=12176\n",
        "\n",
        "Button A: X+17, Y+86\n",
        "Button B: X+84, Y+37\n",
        "Prize: X=7870, Y=6450\n",
        "\n",
        "Button A: X+69, Y+23\n",
        "Button B: X+27, Y+71\n",
        "Prize: X=18641, Y=10279",
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "480");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "0");
    }
}
