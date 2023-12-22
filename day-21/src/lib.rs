use flat::{coordinate::Coordinate, grid::Grid};
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str, steps: usize) -> usize {
    solve(input, steps, true)
}

pub fn part_two(input: &str) -> usize {
    solve(input, 26501365, false)
}

// this algorithm is inspured on this writeup:
//   https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
// which is implemented below with very few differences. my instinct was to use google sheets
// to figure out the pattern, but after staring at data for too long i couldn't put together
// a hypthothesis that worked. i then started working towards a geometric solution, similar
// to the one described above, however, i was unable to resolve the corner squares until this
// write-up outlined it.
pub fn solve(input: &str, steps: usize, part_one: bool) -> usize {
    let grid = Grid::parse(input);
    let grid_size = grid.bounds.width() as usize;
    let mut queue = VecDeque::<(Coordinate, usize)>::new();
    let mut visited = HashMap::new();

    assert_eq!(grid.bounds.width(), grid.bounds.height());
    queue.push_back((grid.find(&'S').unwrap(), 0));

    while let Some((coord, dist)) = queue.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }

        visited.insert(coord, dist);
        coord
            .cardinals()
            .iter()
            .filter(|neighbor| grid.in_bounds(neighbor))
            .filter(|neighbor| !visited.contains_key(neighbor))
            .filter(|neighbor| grid.is_not_equal(neighbor, &'#'))
            .for_each(|neighbor| queue.push_back((*neighbor, dist + 1)));
    }

    if part_one {
        return visited
            .iter()
            .filter(|(_, distance)| **distance <= steps)
            .filter(|(_, distance)| **distance % 2 == 0)
            .count();
    }

    // Our input is 131x131 tiles in size, and 26501365 = 65 + (202300 * 131). 65
    // is the number of steps it takes to get from the centre of the square to the
    // edge, and 131 is the number of steps it takes to traverse the whole square
    let squares_width_traveled = (steps - (grid_size / 2)) / grid_size;
    assert_eq!(squares_width_traveled, 202300); // 2023...cute

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited
        .values()
        .filter(|distance| **distance % 2 == 0)
        .count();
    let odd_full = visited
        .values()
        .filter(|distance| **distance % 2 == 1)
        .count();

    let even_full_visited = squares_width_traveled * squares_width_traveled;
    let odd_full_visited = (squares_width_traveled + 1) * (squares_width_traveled + 1);

    (odd_full_visited * odd_full) + (even_full_visited * even_full)
        - ((squares_width_traveled + 1) * odd_corners)
        + (squares_width_traveled * even_corners)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(16, part_one(EXAMPLE, 6));
        assert_eq!(3615, part_one(INPUT, 64));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(602259568764234, part_two(INPUT));
    }
}
