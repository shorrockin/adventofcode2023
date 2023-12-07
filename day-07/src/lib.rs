use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn calculate_winnings(input: &str, jokers_wild: bool) -> i32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| Hand::new(line, jokers_wild))
        .collect();
    hands.sort_by(|a, b| match a.hand_strength.cmp(&b.hand_strength) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        ordering => ordering,
    });
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid_amount * (rank as i32 + 1))
        .sum()
}

struct Hand {
    cards: Vec<i32>,
    hand_strength: HandStrength,
    bid_amount: i32,
}
impl Hand {
    fn new(input: &str, jokers_wild: bool) -> Hand {
        let (cards_str, bid_amount) = input.split_once(' ').unwrap();
        let cards: Vec<_> = cards_str
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => match jokers_wild {
                    true => 0,
                    false => 11,
                },
                'T' => 10,
                _ => c.to_digit(10).unwrap() as i32,
            })
            .collect();

        let joker_count = cards.iter().filter(|c| **c == 0).count();

        let binding = cards.iter().fold(HashMap::new(), |mut acc, card| {
            if 0 != *card {
                *acc.entry(card).or_insert(0) += 1;
            }
            acc
        });
        let counts: Vec<_> = binding
            .values()
            .filter(|c| **c != 1)
            .sorted()
            .rev()
            .collect();

        let hand_strength = match counts.as_slice() {
            [5] => HandStrength::FiveOfAKind.add_jokers(joker_count),
            [4] => HandStrength::FourOfAKind.add_jokers(joker_count),
            [3, 2] => HandStrength::FullHouse.add_jokers(joker_count),
            [3] => HandStrength::ThreeOfAKind.add_jokers(joker_count),
            [2, 2] => HandStrength::TwoPair.add_jokers(joker_count),
            [2] => HandStrength::OnePair.add_jokers(joker_count),
            _ => HandStrength::HighCard.add_jokers(joker_count),
        };

        Hand {
            cards,
            hand_strength,
            bid_amount: bid_amount.parse().unwrap(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum HandStrength {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
impl HandStrength {
    fn add_jokers(&self, amount: usize) -> HandStrength {
        if amount == 0 {
            return *self;
        }

        // this could be more clever, but this is also explicit, handle the complexity
        // around add jokers to something that could be a full house, or a four of kind
        match self {
            HandStrength::FourOfAKind => match amount {
                1 => HandStrength::FiveOfAKind,
                _ => panic!("can't add more than 1 jokers to a four of a kind"),
            },
            HandStrength::ThreeOfAKind => match amount {
                1 => HandStrength::FourOfAKind,
                2 => HandStrength::FiveOfAKind,
                _ => panic!("can't add more than 2 jokers to a three of a kind"),
            },
            HandStrength::TwoPair => match amount {
                1 => HandStrength::FullHouse,
                _ => panic!("can't add more than 1 jokers to a two pair"),
            },
            HandStrength::OnePair => match amount {
                1 => HandStrength::ThreeOfAKind,
                2 => HandStrength::FourOfAKind,
                3 => HandStrength::FiveOfAKind,
                _ => panic!("can't add more than 3 jokers to a one pair"),
            },
            HandStrength::HighCard => match amount {
                1 => HandStrength::OnePair,
                2 => HandStrength::ThreeOfAKind,
                3 => HandStrength::FourOfAKind,
                4 => HandStrength::FiveOfAKind,
                5 => HandStrength::FiveOfAKind,
                _ => panic!("can't add more than 5 jokers to a high card"),
            },
            _ => panic!("can't add a joker to a full hand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("input.example.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one() {
        assert_eq!(6440, calculate_winnings(EXAMPLE, false));
        assert_eq!(253866470, calculate_winnings(INPUT, false));
    }

    #[test]
    fn part_two() {
        assert_eq!(5905, calculate_winnings(EXAMPLE, true));
        assert_eq!(254494947, calculate_winnings(INPUT, true));
    }
}
