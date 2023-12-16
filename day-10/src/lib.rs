use flat::{
    coordinate::Coordinate,
    coordinate::Direction::{self, East, North, South, West},
    grid::Grid,
};
use std::collections::HashSet;
use std::str::FromStr;

const START_CHAR: char = 'S';

pub fn part_one(input: &str, start_direction: Direction) -> usize {
    generate_path(&Grid::from_str(input).unwrap(), start_direction).len() / 2
}

// start direction should be specified such that the "right" of the path points
// to the inward loop. we could of course write code to figure this out, but it's
// also trivial to pass it in by looking at our dataset.
pub fn part_two(input: &str, start_direction: Direction) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let path = generate_path(&grid, start_direction);
    let path_coordinates = path.iter().map(|(coord, _)| *coord).collect::<HashSet<_>>();

    let mut processed: HashSet<Coordinate> = HashSet::new();
    let mut to_process: HashSet<Coordinate> = path
        .iter()
        .flat_map(|(coord, direction)| {
            inner_points(coord, &grid, *direction)
                .iter()
                .map(|offset| *coord + *offset)
                .filter(|c| !path_coordinates.contains(c))
                .collect::<Vec<_>>()
        })
        .collect();

    // flood the contained points, spreading to neighbors, then filtering either
    // things we've already seen or the pathing loop
    while !to_process.is_empty() {
        processed.extend(to_process.clone());
        let neighbors = to_process
            .iter()
            .flat_map(|coord| coord.cardinals())
            .filter(|neighbor| grid.get(neighbor).is_some())
            .filter(|neighbor| !path_coordinates.contains(neighbor))
            .collect::<Vec<_>>();

        to_process = neighbors
            .iter()
            .filter(|neighbor| !processed.contains(neighbor))
            .copied()
            .collect();
    }

    processed.len()
}

fn generate_path(grid: &Grid, start_direction: Direction) -> Vec<(Coordinate, Direction)> {
    let mut current = grid.find(&START_CHAR).unwrap() + start_direction;
    let mut in_direction = start_direction.invert();
    let mut path = vec![];

    loop {
        path.push((current, in_direction));
        if grid.get(&current).unwrap() == &START_CHAR {
            break;
        }
        let out_direction = out_direction(&current, grid, in_direction);
        current = current + out_direction;
        in_direction = out_direction.invert();
    }

    path
}

pub fn out_direction(coordinate: &Coordinate, grid: &Grid, in_direction: Direction) -> Direction {
    match (in_direction, grid.get(coordinate).unwrap()) {
        (_, '|') => in_direction.invert(),
        (_, '-') => in_direction.invert(),
        (North, 'L') => East,
        (East, 'L') => North,
        (North, 'J') => West,
        (West, 'J') => North,
        (South, '7') => West,
        (West, '7') => South,
        (South, 'F') => East,
        (East, 'F') => South,
        _ => panic!("invalid character, cannot resolve"),
    }
}

pub fn inner_points(
    coordinate: &Coordinate,
    grid: &Grid,
    in_direction: Direction,
) -> Vec<Direction> {
    match (in_direction, grid.get(coordinate).unwrap()) {
        (North, '|') => vec![West],
        (South, '|') => vec![East],
        (East, '-') => vec![North],
        (West, '-') => vec![South],
        (North, 'L') => vec![West, South],
        (East, 'L') => vec![],
        (North, 'J') => vec![],
        (West, 'J') => vec![South, East],
        (South, '7') => vec![East, North],
        (West, '7') => vec![],
        (South, 'F') => vec![],
        (East, 'F') => vec![North, West],
        (_, 'S') => vec![],
        _ => panic!("invalid character for inner points, cannot resolve"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = include_str!("input.example.one.txt");
    const EXAMPLE_TWO: &str = include_str!("input.example.two.txt");
    const EXAMPLE_THREE: &str = include_str!("input.example.three.txt");
    const EXAMPLE_FOUR: &str = include_str!("input.example.four.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(8, part_one(EXAMPLE_ONE, South));
        assert_eq!(7012, part_one(INPUT, East));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two(EXAMPLE_TWO, East));
        assert_eq!(8, part_two(EXAMPLE_THREE, South));
        assert_eq!(10, part_two(EXAMPLE_FOUR, South));
        assert_eq!(395, part_two(INPUT, East));
    }
}
