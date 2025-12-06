//! # Day 6: Trash Compactor
//!
//! The input has columns of math problems consisting of three numbers and an
//! operator which explains whether the numbers should be added (+) or
//! multiplied (*). The solution is the sum of all the individual solutions.
//!
//! [puzzle site](https://adventofcode.com/2025/day/6)

fn get_dimensions<E>(lines: &[Vec<E>]) -> Result<[usize; 2], String> {
    let height = lines.len();
    if height < 2 {
        Err("too few lines")?
    }
    let width = lines[0].len();
    if !lines.iter().map(|line| line.len()).all(|len| len == width) {
        Err("lines have different lengths")?
    }
    Ok([width, height])
}

/// Day 1
#[allow(clippy::needless_range_loop)]
pub fn part1(input: String) -> crate::PuzzleResult {
    let lines: Vec<Vec<&str>> =
        (input.lines().map(|line| line.split_whitespace().collect())).collect();
    let [width, height] = get_dimensions(&lines)?;
    let mut sum = 0;
    for j in 0..width {
        let operator = lines[height - 1][j];
        let mut result: u64 = lines[0][j].parse()?;
        for i in 1..(height - 1) {
            result = match operator {
                "+" => result + lines[i][j].parse::<u64>()?,
                "*" => result * lines[i][j].parse::<u64>()?,
                _ => Err("invalid operator")?,
            };
        }
        sum += result;
    }
    Ok(sum.to_string())
}

/// Day 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let lines: Vec<Vec<char>> = (input.lines().map(|line| line.chars().collect())).collect();
    let [width, height] = get_dimensions(&lines)?;
    let mut sum: u64 = 0;
    let mut start_new_problem = true;
    let mut operator = ' ';
    let mut numbers = Vec::new();
    for j in 0..width {
        if start_new_problem {
            operator = lines[height - 1][j];
            numbers.clear();
            start_new_problem = false
        }
        let mut number = String::new();
        for line in &lines[..(height - 1)] {
            if line[j].is_ascii_digit() {
                number.push(line[j]);
            }
        }
        if !number.is_empty() {
            numbers.push(number.parse()?);
        }
        if number.is_empty() || j == width - 1 {
            sum += match operator {
                '+' => (numbers.iter().copied()).reduce(|result, number| result + number),
                '*' => (numbers.iter().copied()).reduce(|result, number| result * number),
                _ => Err("invalid operator")?,
            }
            .unwrap_or(0);
            start_new_problem = true;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "3263827");
    }
}
