use flat::{
    coordinate::{
        Coordinate,
        Direction::{self, East, North, South, West},
    },
    grid::Grid,
};
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    count_energized(&Grid::parse(input), Coordinate(0, 0), East)
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::parse(input);
    let bounds = grid.bounds;

    (bounds.x.min..=bounds.x.max)
        .map(|x| ((x, bounds.y.min), South))
        .chain((bounds.x.min..=bounds.x.max).map(|x| ((x, bounds.y.max), North)))
        .chain((bounds.y.min..=bounds.y.max).map(|y| ((bounds.x.min, y), East)))
        .chain((bounds.y.min..=bounds.y.max).map(|y| ((bounds.x.max, y), West)))
        .map(|(c, dir)| count_energized(&grid, Coordinate(c.0, c.1), dir))
        .max()
        .unwrap()
}

fn count_energized(grid: &Grid, starting: Coordinate, direction: Direction) -> usize {
    let mut energized: HashSet<Laser> = HashSet::new();
    let mut lasers = vec![Laser::new(starting, direction)];
    let mut split_lasers: Vec<Laser> = vec![];

    while !lasers.is_empty() {
        for laser in lasers.iter_mut() {
            energized.insert(laser.clone());

            laser.direction = match grid.get(&laser.position) {
                Some('.') => laser.direction,
                Some('\\') => match laser.direction {
                    North => West,
                    South => East,
                    East => South,
                    West => North,
                    _ => panic!("unknown direction {:?}", laser.direction),
                },
                Some('/') => match laser.direction {
                    North => East,
                    South => West,
                    East => North,
                    West => South,
                    _ => panic!("unknown direction {:?}", laser.direction),
                },
                Some('|') => match laser.direction {
                    East | West => {
                        split_lasers.push(Laser::new(laser.position + North, North));
                        South
                    }
                    _ => laser.direction,
                },
                Some('-') => match laser.direction {
                    North | South => {
                        split_lasers.push(Laser::new(laser.position + East, East));
                        West
                    }
                    _ => laser.direction,
                },
                _ => panic!("unknown character at {:?}", laser.position),
            };

            laser.position = laser.position + laser.direction;
        }

        lasers.append(&mut split_lasers);
        lasers.retain(|l| grid.in_bounds(&l.position));
        lasers.retain(|l| !energized.contains(l));
    }

    energized.iter().map(|l| l.position).unique().count()
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Laser {
    position: Coordinate,
    direction: Direction,
}
impl Laser {
    fn new(position: Coordinate, direction: Direction) -> Laser {
        Laser {
            position,
            direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(46, part_one(EXAMPLE));
        assert_eq!(7034, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(51, part_two(EXAMPLE));
        // a little slow to include by default as part of comprehensive test suite ~1.5s
        //assert_eq!(7759, part_two(INPUT));
    }
}
