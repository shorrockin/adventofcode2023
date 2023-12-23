use flat::coordinate::Coordinate;
use flat::coordinate::Direction::{East, North, South, West};
use flat::grid::Grid;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> usize {
    solve(Grid::parse(input))
}

pub fn part_two(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.replace_char('<', '.');
    grid.replace_char('>', '.');
    grid.replace_char('^', '.');
    grid.replace_char('v', '.');
    solve(grid)
}

pub fn solve(grid: Grid) -> usize {
    let start = Coordinate(1, 0);
    let end = Coordinate(grid.bounds.x.max - 1, grid.bounds.y.max);

    // pre-compute neighbors to memoize this logic a bit
    let mut neighbors: HashMap<_, _> = grid
        .points
        .iter()
        .filter(|(_, symbol)| **symbol != '#')
        .map(|(coord, symbol)| {
            (
                coord,
                match symbol {
                    '>' => vec![(*coord + East, 1)],
                    '<' => vec![(*coord + West, 1)],
                    '^' => vec![(*coord + North, 1)],
                    'v' => vec![(*coord + South, 1)],
                    '.' => coord
                        .cardinals()
                        .iter()
                        .filter(|c| grid.in_bounds(c))
                        .filter(|coord| grid.is_not_equal(coord, &'#'))
                        .copied()
                        .map(|c| (c, 1))
                        .collect(),
                    _ => panic!("unexpected char: {}", symbol),
                },
            )
        })
        .collect();

    // prune the true, collapsing all paths that are in a cooridor
    // into a single step, this could be optimized, but it's also
    // straight forward, so it'll do for now
    loop {
        let to_prune_option = neighbors
            .iter()
            .find(|(_, v)| v.len() == 2)
            .map(|(k, v)| (*k, v.clone()));
        if to_prune_option.is_none() {
            break;
        }

        let to_prune = to_prune_option.unwrap();
        let (left, left_dist) = to_prune.1[0];
        let (right, right_dist) = to_prune.1[1];

        neighbors
            .get_mut(&left)
            .unwrap()
            .retain(|(k, _)| k != to_prune.0);
        neighbors
            .get_mut(&left)
            .unwrap()
            .push((right, left_dist + right_dist));
        neighbors
            .get_mut(&right)
            .unwrap()
            .retain(|(k, _)| k != to_prune.0);
        neighbors
            .get_mut(&right)
            .unwrap()
            .push((left, left_dist + right_dist));
        neighbors.remove(to_prune.0);
    }

    // simple dfs using queue of position, visited nodes & steps
    let mut max_steps = 0;
    let mut queue = VecDeque::new();
    queue.push_front((start, HashSet::<Coordinate>::new(), 0));

    while let Some((coordinate, mut visited, steps)) = queue.pop_front() {
        visited.insert(coordinate);
        if steps > max_steps && coordinate == end {
            max_steps = steps;
        }

        neighbors
            .get(&coordinate)
            .unwrap()
            .iter()
            .filter(|(coord, _)| !visited.contains(coord))
            .for_each(|(coord, dist)| queue.push_front((*coord, visited.clone(), steps + dist)));
    }

    max_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(94, part_one(EXAMPLE));
        assert_eq!(2430, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(154, part_two(EXAMPLE));
        // to slow for normal test suite, ~5s when run in release mode
        //assert_eq!(6534, part_two(INPUT));
    }
}
