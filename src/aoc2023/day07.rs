//! # Day 7: Camel Cards
//!
//! Lines start with five characters representing a hand of cards and end with a
//! number representing a bid. The hands need to be ordered by first comparing
//! their kinds and whenever they are the same kind then by comparing their
//! cards first to last. The solution is the sum of all bids multiplied by their
//! rank.

#[derive(Clone, Eq, Hash, Ord, PartialOrd, PartialEq)]
enum Card {
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

impl TryFrom<char> for Card {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            c => Err(format!("{c} is not a card"))?,
        })
    }
}

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
        for card in &self.cards {
            counts
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut counts: Vec<&usize> = counts.values().collect();
        counts.sort();
        counts.reverse();
        match *counts[0] {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 if *counts[1] == 2 => FullHouse,
            3 => ThreeOfAKind,
            2 if *counts[1] == 2 => TwoPair,
            2 => OnePair,
            1 => HighCard,
            c => panic!("count of {c} should be impossible"),
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

impl std::str::FromStr for Hand {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").ok_or("not a hand")?;
        let mut cards = cards.chars().map(|c| c.try_into());
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
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let mut hands = Vec::<Hand>::new();
    for line in input.lines() {
        hands.push(line.parse()?);
    }
    hands.sort();
    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index + 1) * hand.bid;
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483"
            .to_string();
        assert_eq!(&super::part1(input).unwrap(), "6440");
    }
}
