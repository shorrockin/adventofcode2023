use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    parse(input).iter().map(|card| card.score).sum()
}

pub fn part_two(input: &str) -> u32 {
    let cards = parse(input);
    let mut counts: HashMap<_, _> = cards.iter().map(|card| (card.id, 1)).collect();

    for original_card in cards {
        for won_card in original_card.id + 1..=original_card.id + original_card.matches {
            counts.insert(
                won_card,
                counts.get(&won_card).unwrap() + counts.get(&original_card.id).unwrap(),
            );
        }
    }

    counts.values().sum()
}

fn parse(input: &str) -> Vec<Card> {
    input.lines().map(Card::new).collect()
}

struct Card {
    id: u32,
    matches: u32,
    score: u32,
}
impl Card {
    fn new(input: &str) -> Card {
        let (card, numbers) = input.split_once(": ").unwrap();
        let (_, id) = card.split_once(' ').unwrap();
        let (winning_numbers_str, numbers_str) = numbers.split_once(" | ").unwrap();

        let numbers: Vec<u32> = numbers_str
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let winning_numbers: Vec<u32> = winning_numbers_str
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let matches: u32 = numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count()
            .try_into()
            .unwrap();

        let score = match matches {
            0 => 0,
            _ => 1 << (matches - 1),
        };

        Card {
            id: id.trim().parse().unwrap(),
            matches,
            score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE_ONE: &str = include_str!("input.example.one.txt");
    static EXAMPLE_TWO: &str = include_str!("input.example.two.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(13, part_one(EXAMPLE_ONE));
        assert_eq!(26426, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(30, part_two(EXAMPLE_TWO));
        assert_eq!(6227972, part_two(INPUT));
    }
}
