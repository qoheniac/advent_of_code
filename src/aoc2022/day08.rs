//! # Day 8: Treetop Tree House
//!
//! [puzzle site](https://adventofcode.com/2022/day/8)

type Forrest = Vec<Vec<u32>>;

fn read_forrest(input: String) -> Result<Forrest, String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10))
                .collect()
        })
        .collect::<Option<Forrest>>()
        .ok_or("couldn't read forrest".to_string())
}

fn update_reached_threshold(
    max_height: &mut Option<u32>,
    is_visible: &mut Vec<Vec<bool>>,
    forrest: &Forrest,
    (row_index, column_index): (usize, usize),
) -> bool {
    let height = Some(forrest[row_index][column_index]);
    if height > *max_height {
        *max_height = height;
        is_visible[row_index][column_index] = true;
    }
    *max_height == Some(9)
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let forrest = read_forrest(input)?;
    let row_number = forrest.len();
    let column_number = forrest[0].len();
    let mut is_visible = vec![vec![false; column_number]; row_number];

    // look from left
    for i in 0..row_number {
        let mut max_height = None;
        for j in 0..column_number {
            if update_reached_threshold(&mut max_height, &mut is_visible, &forrest, (i, j)) {
                break;
            }
        }
    }

    // look from top
    for j in 0..column_number {
        let mut max_height = None;
        for i in 0..row_number {
            if update_reached_threshold(&mut max_height, &mut is_visible, &forrest, (i, j)) {
                break;
            }
        }
    }

    // look from right
    for i in 0..row_number {
        let mut max_height = None;
        for j in (0..column_number).rev() {
            if update_reached_threshold(&mut max_height, &mut is_visible, &forrest, (i, j)) {
                break;
            }
        }
    }

    // look from bottom
    for j in 0..column_number {
        let mut max_height = None;
        for i in (0..row_number).rev() {
            if update_reached_threshold(&mut max_height, &mut is_visible, &forrest, (i, j)) {
                break;
            }
        }
    }

    // count visible trees
    let count = is_visible.into_iter().flatten().filter(|b| *b).count();
    Ok(count.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let forrest = read_forrest(input)?;
    let row_number = forrest.len();
    let column_number = forrest[0].len();
    let mut max_score = 0;
    for i in 0..row_number {
        for j in 0..column_number {
            let height = forrest[i][j];

            // look up
            let mut up_score = 0;
            for k in (0..i).rev() {
                up_score += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            // look left
            let mut left_score = 0;
            for k in (0..j).rev() {
                left_score += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }

            // look right
            let mut right_score = 0;
            for k in (j + 1)..column_number {
                right_score += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }

            // look down
            let mut down_score = 0;
            for k in (i + 1)..row_number {
                down_score += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            let score = up_score * left_score * right_score * down_score;
            if score > max_score {
                max_score = score;
            }
        }
    }
    Ok(max_score.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "8");
    }
}
