//! # Day 13: Distress Signal
//!
//! [puzzle site](https://adventofcode.com/2022/day/13)

#[derive(PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Self>),
}

impl Packet {
    fn new(packet_string: &str) -> Self {
        let mut chars = packet_string.chars();
        if chars.next().unwrap() == '[' {
            if chars.next().unwrap() == ']' {
                Self::List(Vec::new())
            } else {
                let length = packet_string.len();
                let list_body = &packet_string[1..length - 1];
                let mut list = Vec::new();
                let mut depth = 0;
                let mut left = 0;
                for (index, character) in list_body.char_indices() {
                    match character {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        ',' => {
                            if depth == 0 {
                                list.push(Self::new(&list_body[left..index]));
                                left = index + 1;
                            }
                        }
                        _ => (),
                    }
                }
                list.push(Self::new(&list_body[left..]));
                Self::List(list)
            }
        } else {
            Self::Integer(packet_string.parse().unwrap())
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(self_num), Self::Integer(other_num)) => self_num.cmp(other_num),
            (Self::Integer(self_num), Self::List(_)) => {
                Self::List(vec![Self::Integer(*self_num)]).cmp(other)
            }
            (Self::List(_), Self::Integer(other_num)) => {
                self.cmp(&Self::List(vec![Self::Integer(*other_num)]))
            }
            (Self::List(self_list), Self::List(other_list)) => self_list.cmp(other_list),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    Ok(input
        .split("\n\n")
        .enumerate()
        .map(|(index, pair)| {
            let mut packets = pair.lines();
            let packet1 = Packet::new(packets.next().unwrap());
            let packet2 = Packet::new(packets.next().unwrap());
            (packet1 < packet2) as usize * (index + 1)
        })
        .sum::<usize>()
        .to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::new)
        .collect();
    packets.sort();
    let divider1 = Packet::new("[[2]]");
    let divider2 = Packet::new("[[6]]");
    let mut decoder_key = 0;
    for (index, packet) in packets.iter().enumerate() {
        if decoder_key == 0 && packet > &divider1 {
            decoder_key = index + 1;
        }
        if packet > &divider2 {
            decoder_key *= index + 2;
            break;
        }
    }
    Ok(decoder_key.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "140");
    }
}
