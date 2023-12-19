//! # Day 7: Camel Cards
//!
//! Lines start with five characters representing a hand of cards and end with a
//! number representing a bid. The hands need to be ordered by first comparing
//! their kinds and whenever they are the same kind then by comparing their
//! cards first to last. The solution is the sum of all bids multiplied by their
//! rank.
//!
//! [puzzle site](https://adventofcode.com/2023/day/7)

#[derive(Clone, Eq, Hash, Ord, PartialOrd, PartialEq)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
use Card::*;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
use Kind::*;

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn kind(&self) -> Kind {
        let mut counts = std::collections::HashMap::new();
        let mut jokers = 0;
        for card in &self.cards {
            if *card == Joker {
                jokers += 1;
            } else {
                counts
                    .entry(card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        if jokers == 5 {
            return FiveOfAKind;
        }
        let mut counts: Vec<usize> = counts.into_values().collect();
        counts.sort();
        counts.reverse();
        counts[0] += jokers;
        match counts[0] {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 if counts[1] == 2 => FullHouse,
            3 => ThreeOfAKind,
            2 if counts[1] == 2 => TwoPair,
            2 => OnePair,
            1 => HighCard,
            c => unreachable!("count of {c} should be impossible"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First rule: sort by kind
        let kind_order = self.kind().cmp(&other.kind());
        if kind_order.is_ne() {
            return kind_order;
        }
        // Second rule: sort by card
        for (this_card, other_card) in self.cards.iter().zip(&other.cards) {
            let card_order = this_card.cmp(other_card);
            if card_order.is_ne() {
                return card_order;
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

enum Rules {
    WithJokers,
    WithoutJokers,
}
use Rules::*;

impl Rules {
    fn try_card_from_char(&self, c: char) -> Result<Card, String> {
        Ok(match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => match self {
                WithJokers => Joker,
                WithoutJokers => Jack,
            },
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => Err(format!("{c} is not a card"))?,
        })
    }

    fn try_hand_from_str(&self, s: &str) -> Result<Hand, Box<dyn std::error::Error>> {
        let (cards, bid) = s.split_once(" ").ok_or("not a hand")?;
        let mut cards = cards.chars().map(|c| self.try_card_from_char(c));
        let error = "too few cards";
        let cards = [
            cards.next().ok_or(error)??,
            cards.next().ok_or(error)??,
            cards.next().ok_or(error)??,
            cards.next().ok_or(error)??,
            cards.next().ok_or(error)??,
        ];
        let bid = bid.parse()?;
        Ok(Hand { cards, bid })
    }

    fn solve_puzzle(&self, input: String) -> crate::PuzzleResult {
        let mut hands = Vec::<Hand>::new();
        for line in input.lines() {
            hands.push(self.try_hand_from_str(line)?);
        }
        hands.sort();
        let mut total = 0;
        for (index, hand) in hands.iter().enumerate() {
            total += (index + 1) * hand.bid;
        }
        Ok(total.to_string())
    }
}

/// Part 1: Without jokers
pub fn part1(input: String) -> crate::PuzzleResult {
    WithoutJokers.solve_puzzle(input)
}

/// Part 2: With jokers
pub fn part2(input: String) -> crate::PuzzleResult {
    WithJokers.solve_puzzle(input)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "6440");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "5905");
    }
}
