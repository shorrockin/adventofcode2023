use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    sets: Vec<HashMap<String, usize>>,
}
impl Game {
    pub fn new(input: &str) -> Game {
        let (game_id, results) = input.split_once(':').unwrap();

        Game {
            id: game_id
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            sets: results
                .split(';')
                .map(|set| {
                    set.split(',')
                        .map(|cube| {
                            let (amount, color) = cube.trim().split_once(' ').unwrap();
                            (color.to_string(), amount.parse::<usize>().unwrap())
                        })
                        .collect::<HashMap<String, usize>>()
                })
                .collect::<Vec<HashMap<String, usize>>>(),
        }
    }

    pub fn contains_set(&self, tester: &HashMap<String, usize>) -> bool {
        self.sets.len()
            == self
                .sets
                .iter()
                .filter(|set| {
                    set.iter()
                        .filter(|(color, amount)| {
                            tester.contains_key(*color) && tester.get(*color).unwrap() >= amount
                        })
                        .count()
                        == set.len()
                })
                .count()
    }

    pub fn power(&self) -> usize {
        self.sets
            .iter()
            .fold(HashMap::new(), |mut acc, set| {
                set.iter().for_each(|(color, amount)| {
                    if acc.contains_key(color) {
                        let current: usize = *acc.get(color).unwrap();
                        acc.insert(color.to_string(), current.max(*amount));
                    } else {
                        acc.insert(color.to_string(), *amount);
                    }
                });
                acc
            })
            .values()
            .product()
    }
}

pub fn sum_games_with_sets(input: &str, set: HashMap<String, usize>) -> usize {
    input
        .lines()
        .map(Game::new)
        .collect::<Vec<Game>>()
        .iter()
        .filter(|game| game.contains_set(&set))
        .map(|game| game.id)
        .sum()
}

pub fn sum_game_powers(input: &str) -> usize {
    input
        .lines()
        .map(Game::new)
        .collect::<Vec<Game>>()
        .iter()
        .map(|game| game.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    fn part_one_test_set() -> HashMap<String, usize> {
        HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ])
    }

    #[test]
    fn part_one() {
        assert_eq!(8, sum_games_with_sets(EXAMPLE, part_one_test_set()));
        assert_eq!(2545, sum_games_with_sets(INPUT, part_one_test_set()));
    }

    #[test]
    fn part_two() {
        assert_eq!(2286, sum_game_powers(EXAMPLE));
        assert_eq!(78111, sum_game_powers(INPUT));
    }
}
