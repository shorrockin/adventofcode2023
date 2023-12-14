use flat::coordinate::offsets::{EAST, NORTH, SOUTH, WEST};
use flat::coordinate::Coordinate;
use flat::grid::Grid;
use std::collections::HashMap;
use std::str::FromStr;

const ROCK: char = 'O';
const WALL: char = '#';
const CYCLE_COUNT: i32 = 1_000_000_000;

enum Tilt {
    North,
    South,
    East,
    West,
}

pub fn part_one(input: &str) -> i32 {
    score(&tilt(parse(input), Tilt::North))
}

pub fn part_two(input: &str) -> i32 {
    let mut grid = parse(input);
    let mut current_cycle = 0;
    let mut cycle_cache: HashMap<u64, i32> = HashMap::new();

    while current_cycle < CYCLE_COUNT {
        grid = tilt(grid, Tilt::North);
        grid = tilt(grid, Tilt::West);
        grid = tilt(grid, Tilt::South);
        grid = tilt(grid, Tilt::East);
        current_cycle += 1;

        let cycle_hash = grid.hash();

        if let Some(cycle_at) = cycle_cache.get(&cycle_hash) {
            let loop_size = current_cycle - cycle_at;
            let remaining = CYCLE_COUNT - current_cycle;
            current_cycle += (remaining / loop_size) * loop_size;
        }

        cycle_cache.insert(cycle_hash, current_cycle);
    }

    score(&grid)
}

fn parse(input: &str) -> Grid {
    let mut grid = Grid::from_str(input).unwrap();
    grid.remove_char('.');
    grid
}

fn tilt(mut grid: Grid, tilt: Tilt) -> Grid {
    let (starting_point, side_direction, rock_direction) = match tilt {
        Tilt::North => (Coordinate(0, 0), EAST, SOUTH),
        Tilt::South => (Coordinate(0, grid.bounds.y.max), EAST, NORTH),
        Tilt::West => (Coordinate(0, 0), SOUTH, EAST),
        Tilt::East => (Coordinate(grid.bounds.x.max, 0), SOUTH, WEST),
    };

    let mut side_position = starting_point;

    while grid.in_bounds(&side_position) {
        let mut fall_to: Option<Coordinate> = None;
        let mut rock_position = side_position;

        while grid.in_bounds(&rock_position) {
            if grid.is_empty(&rock_position) && fall_to.is_none() {
                fall_to = Some(rock_position);
            }

            if grid.is_equal(&rock_position, &WALL) {
                fall_to = None;
            }

            if grid.is_equal(&rock_position, &ROCK) && fall_to.is_some() {
                grid.move_point(rock_position, fall_to.unwrap());
                fall_to = Some(fall_to.unwrap() + rock_direction);
            }

            rock_position = rock_position + rock_direction;
        }

        side_position = side_position + side_direction;
    }

    grid
}

fn score(grid: &Grid) -> i32 {
    grid.find_all(&ROCK)
        .iter()
        .map(|coord| grid.bounds.y.max - coord.1 + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(136, part_one(EXAMPLE));
        assert_eq!(105249, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(64, part_two(EXAMPLE));
        // takes a ~2 seconds to run, too slow to include in normal test suite,
        // in release mode takes 0.01s
        // assert_eq!(88680, part_two(INPUT));
    }
}
