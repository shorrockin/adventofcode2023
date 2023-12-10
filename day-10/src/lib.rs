use flat::{
    coordinate::offsets::{EAST, NORTH, SOUTH, WEST},
    coordinate::Coordinate,
    coordinate::Offset,
    grid::Grid,
};
use std::collections::HashSet;
use std::str::FromStr;

const START_CHAR: char = 'S';

pub fn part_one(input: &str, start_direction: Offset) -> usize {
    generate_path(&Grid::from_str(input).unwrap(), start_direction).len() / 2
}

// start direction should be specified such that the "right" of the path points
// to the inward loop. we could of course write code to figure this out, but it's
// also trivial to pass it in by looking at our dataset.
pub fn part_two(input: &str, start_direction: Offset) -> usize {
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

fn generate_path(grid: &Grid, start_direction: Offset) -> Vec<(Coordinate, Offset)> {
    let mut current = grid.find(START_CHAR).unwrap() + start_direction;
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

pub fn out_direction(coordinate: &Coordinate, grid: &Grid, in_direction: Offset) -> Offset {
    match (in_direction, grid.get(coordinate).unwrap()) {
        (_, '|') => in_direction.invert(),
        (_, '-') => in_direction.invert(),
        // can't use offsets constants here, as much as it would read better 🤷
        (Offset(0, -1), 'L') => EAST,
        (Offset(1, 0), 'L') => NORTH,
        (Offset(0, -1), 'J') => WEST,
        (Offset(-1, 0), 'J') => NORTH,
        (Offset(0, 1), '7') => WEST,
        (Offset(-1, 0), '7') => SOUTH,
        (Offset(0, 1), 'F') => EAST,
        (Offset(1, 0), 'F') => SOUTH,
        _ => panic!("invalid character, cannot resolve"),
    }
}

pub fn inner_points(coordinate: &Coordinate, grid: &Grid, in_direction: Offset) -> Vec<Offset> {
    match (in_direction, grid.get(coordinate).unwrap()) {
        (Offset(0, -1), '|') => vec![WEST],        // from the north
        (Offset(0, 1), '|') => vec![EAST],         // from the south
        (Offset(1, 0), '-') => vec![NORTH],        // from the east
        (Offset(-1, 0), '-') => vec![SOUTH],       // from the west
        (Offset(0, -1), 'L') => vec![WEST, SOUTH], // from the north
        (Offset(1, 0), 'L') => vec![],             // from the east (corner scenario)
        (Offset(0, -1), 'J') => vec![],            // from the north (corner scenario)
        (Offset(-1, 0), 'J') => vec![SOUTH, EAST], // from the west
        (Offset(0, 1), '7') => vec![EAST, NORTH],  // from the south
        (Offset(-1, 0), '7') => vec![],            // from the west (corner scenario)
        (Offset(0, 1), 'F') => vec![],             // from the south (corner scenario)
        (Offset(1, 0), 'F') => vec![NORTH, WEST],  // from the east
        (_, 'S') => vec![],                        // ignore starting point
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
        assert_eq!(8, part_one(EXAMPLE_ONE, SOUTH));
        assert_eq!(7012, part_one(INPUT, EAST));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two(EXAMPLE_TWO, EAST));
        assert_eq!(8, part_two(EXAMPLE_THREE, SOUTH));
        assert_eq!(10, part_two(EXAMPLE_FOUR, SOUTH));
        assert_eq!(395, part_two(INPUT, EAST));
    }
}
