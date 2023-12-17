use flat::coordinate::Coordinate;
use flat::coordinate::Direction::{self, East, South};
use flat::grid::Grid;
use pathfinding::prelude::dijkstra;

pub fn part_one(input: &str) -> u32 {
    solve(input, 0, 3)
}

pub fn part_two(input: &str) -> u32 {
    solve(input, 4, 10)
}

fn solve(input: &str, min_momentum: u32, max_momentum: u32) -> u32 {
    let grid = Grid::parse(input);
    let starting = Node::new(grid.bounds.min(), East, 0);
    let destination = grid.bounds.max();

    let (_, cost) = dijkstra(
        &starting,
        |c| c.neighbors(&grid, min_momentum, max_momentum),
        |c| (c.position == destination && c.momentum >= min_momentum),
    )
    .unwrap();

    cost
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Node {
    position: Coordinate,
    direction: Direction,
    momentum: u32,
}
impl Node {
    fn new(position: Coordinate, direction: Direction, momentum: u32) -> Node {
        Node {
            position,
            direction,
            momentum,
        }
    }

    fn neighbors(&self, grid: &Grid, min_momentum: u32, max_momentum: u32) -> Vec<(Node, u32)> {
        // if this is the starting position we can only east/south, special case
        // this one circumstance
        if self.position == grid.bounds.min() {
            return vec![
                (
                    Node::new(Coordinate(0, 1), South, 1),
                    grid.get(&Coordinate(0, 1)).unwrap().to_digit(10).unwrap(),
                ),
                (
                    Node::new(Coordinate(1, 0), East, 1),
                    grid.get(&Coordinate(1, 0)).unwrap().to_digit(10).unwrap(),
                ),
            ];
        }

        [
            Node::new(
                self.position + self.direction.cardinal_turn_left(),
                self.direction.cardinal_turn_left(),
                1,
            ),
            Node::new(
                self.position + self.direction.cardinal_turn_right(),
                self.direction.cardinal_turn_right(),
                1,
            ),
            Node::new(
                self.position + self.direction,
                self.direction,
                self.momentum + 1,
            ),
        ]
        .iter()
        .filter(|n| grid.is_some(&n.position))
        .filter(|n| n.momentum <= max_momentum)
        .filter(|n| n.direction == self.direction || self.momentum >= min_momentum)
        .map(|n| (*n, grid.get(&n.position).unwrap().to_digit(10).unwrap()))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(102, part_one(EXAMPLE));
        assert_eq!(686, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(94, part_two(EXAMPLE));
        assert_eq!(801, part_two(INPUT));
    }
}
