use flat::coordinate::Coordinate;
use flat::grid::Grid;
use std::str::FromStr;

pub fn part_one(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|grid_str| Grid::from_str(grid_str).unwrap())
        .map(|grid| score(grid, false))
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|grid_str| Grid::from_str(grid_str).unwrap())
        .map(|grid| score(grid, true))
        .sum()
}

fn score(grid: Grid, smudge: bool) -> i32 {
    match reflection_index(&grid, smudge) {
        Some(col) => col,
        None => match reflection_index(&grid.rotate_right(), smudge) {
            Some(row) => row * 100,
            _ => panic!("no reflection found in grid"),
        },
    }
}

fn reflection_index(grid: &Grid, smudge: bool) -> Option<i32> {
    (1..=grid.bounds.x.max).find(|x| {
        let reflection_size = *x.min(&(grid.bounds.x.max - x + 1));
        let differences: usize = (0..=grid.bounds.y.max)
            .map(|y| {
                (1..=reflection_size)
                    .filter(|i| {
                        let left = grid.get(&Coordinate(*x + i - 1, y));
                        let right = grid.get(&Coordinate(*x - i, y));
                        left != right
                    })
                    .count()
            })
            .sum();

        match smudge {
            true => differences == 1,
            false => differences == 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(405, part_one(EXAMPLE));
        assert_eq!(31265, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(400, part_two(EXAMPLE));
        assert_eq!(39359, part_two(INPUT));
    }
}
