use flat::coordinate::Coordinate;
use flat::grid::Bounds;
use flat::grid::Grid;
use std::collections::HashMap;
use strings::rotate_right;

const EMPTY: char = '.';
const GALAXY: char = '#';

pub fn solve(input: &str, expansion_size: usize) -> i64 {
    let grid = parse(input, expansion_size);
    let galaxies = grid.find_all(&GALAXY);

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, galaxy)| {
            galaxies
                .iter()
                .skip(idx)
                .map(|other_galaxy| galaxy.distance(other_galaxy) as i64)
        })
        .sum()
}

fn parse(input: &str, expansion_size: usize) -> Grid {
    let row_offsets = calculate_offsets(input, expansion_size);
    let col_offsets = calculate_offsets(&rotate_right(input), expansion_size);

    let mut points = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            points.insert(
                Coordinate(
                    x as i32 + col_offsets[x] as i32,
                    y as i32 + row_offsets[y] as i32,
                ),
                c,
            );
        }
    }
    Grid {
        points,
        bounds: Bounds::default(),
    }
}

fn calculate_offsets(input: &str, expansion_size: usize) -> Vec<usize> {
    let mut current_offset: usize = 0;
    input
        .lines()
        .map(|line| {
            if line.chars().all(|c| c == EMPTY) {
                current_offset += expansion_size - 1;
            }
            current_offset
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(374, solve(EXAMPLE, 2));
        assert_eq!(9543156, solve(INPUT, 2));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(1030, solve(EXAMPLE, 10));
        assert_eq!(8410, solve(EXAMPLE, 100));
        assert_eq!(625243292686, solve(INPUT, 1_000_000));
    }
}
