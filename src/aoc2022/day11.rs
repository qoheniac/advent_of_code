//! # Day 11: Monkey in the Middle
//!
//! [puzzle site](https://adventofcode.com/2022/day/11)

struct Monkey {
    items: Vec<u64>,
    operator: char,
    operand: String,
    divisor: u64,
    receivers: Vec<usize>,
    inspected: u64,
}

enum Part {
    Part1,
    Part2,
}
use Part::*;

use crate::PuzzleResult;

fn solution(input: String, part: Part) -> PuzzleResult {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey_input| {
            let mut lines = monkey_input.lines();
            lines.next().unwrap();

            let items: Vec<u64> = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect();

            let operation_data: Vec<&str> = lines
                .next()
                .unwrap()
                .split(" = ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .skip(1)
                .collect();

            let divisor: u64 = lines
                .next()
                .unwrap()
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse()
                .unwrap();

            let receivers: Vec<usize> = lines
                .take(2)
                .map(|line| line.split_whitespace().nth(5).unwrap().parse().unwrap())
                .collect();

            Monkey {
                items,
                operator: operation_data[0].chars().next().unwrap(),
                operand: operation_data[1].to_string(),
                divisor,
                receivers,
                inspected: 0,
            }
        })
        .collect();

    let (rounds, relief) = match part {
        Part1 => (20, 3),
        Part2 => (10000, monkeys.iter().map(|monkey| monkey.divisor).product()),
    };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                monkeys[i].inspected += 1;

                // inspect
                let operand = if monkeys[i].operand == "old" {
                    item
                } else {
                    monkeys[i].operand.parse().unwrap()
                };
                item = match monkeys[i].operator {
                    '+' => item + operand,
                    '*' => item * operand,
                    _ => panic!(),
                };
                item = match part {
                    Part1 => item / relief,
                    Part2 => item % relief,
                };

                // throw
                let j = if item.rem_euclid(monkeys[i].divisor) == 0 {
                    monkeys[i].receivers[0]
                } else {
                    monkeys[i].receivers[1]
                };
                monkeys[j].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspected);
    monkeys.reverse();
    Ok((monkeys[0].inspected * monkeys[1].inspected).to_string())
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    solution(input, Part1)
}
/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    solution(input, Part2)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "10605");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "2713310158");
    }
}
