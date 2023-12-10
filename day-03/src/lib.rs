use flat::coordinate::offsets;
use flat::coordinate::Coordinate;
use flat::grid::Grid;
use std::collections::HashSet;

pub fn sum_part_numbers(grid: &Grid) -> u32 {
    PartNumber::from(grid)
        .iter()
        .filter(|pn| pn.is_symbol_adjacent(grid))
        .map(|pn| pn.value)
        .sum()
}

pub fn sum_gear_ratios(grid: &Grid) -> u32 {
    let part_numbers = PartNumber::from(grid);
    let mut sum = 0;

    let gears: Vec<_> = grid
        .points
        .iter()
        .filter(|(_, char)| **char == '*')
        .map(|(coord, _)| coord)
        .collect();

    for gear in gears {
        let around = gear.intercardinals();
        let gear_part_numbers: Vec<_> = part_numbers
            .iter()
            .filter(|pn| around.iter().any(|v| pn.coordinates.contains(v)))
            .cloned()
            .collect();

        if gear_part_numbers.len() == 2 {
            sum += gear_part_numbers.iter().map(|pn| pn.value).product::<u32>();
        }
    }

    sum
}

#[derive(Debug, PartialEq, Clone)]
pub struct PartNumber {
    coordinates: Vec<Coordinate>,
    value: u32,
}

impl PartNumber {
    pub fn from(grid: &Grid) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();
        for (coord, _) in grid.points.iter() {
            if PartNumber::is_first_digit(*coord, grid) {
                let mut str_value = "".to_string();
                let mut position = *coord;
                let mut coordinates = vec![];

                while let Some(c) = grid.get(&position) {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    str_value += c.to_string().as_str();
                    coordinates.push(position);
                    position = position + offsets::EAST;
                }

                let value = str_value.parse().unwrap();
                part_numbers.push(PartNumber { coordinates, value });
            }
        }
        part_numbers
    }

    fn is_first_digit(coord: Coordinate, grid: &Grid) -> bool {
        if let Some(c) = grid.get(&coord) {
            if !c.is_ascii_digit() {
                return false;
            }

            let previous = coord + offsets::WEST;
            if let Some(c) = grid.get(&previous) {
                if c.is_ascii_digit() {
                    return false;
                }
            }

            return true;
        }
        false
    }

    fn around(&self) -> Vec<Coordinate> {
        let mut result = HashSet::new();
        self.coordinates.iter().for_each(|coord| {
            result.extend(coord.intercardinals());
        });
        result.into_iter().collect()
    }

    fn is_symbol_adjacent(&self, grid: &Grid) -> bool {
        for coord in self.around() {
            if let Some(c) = grid.get(&coord) {
                if !c.is_ascii_digit() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flat::coordinate::Coordinate;
    use std::str::FromStr;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    fn parse(input: &str) -> Grid {
        let mut grid = Grid::from_str(input).unwrap();
        grid.remove_char('.');
        grid
    }

    #[test]
    fn example_input() {
        let grid = parse(EXAMPLE);

        assert_eq!('4', *grid.get(&Coordinate(0, 0)).unwrap());
        assert_eq!('6', *grid.get(&Coordinate(1, 0)).unwrap());
        assert_eq!('7', *grid.get(&Coordinate(2, 0)).unwrap());

        let part_numbers = PartNumber::from(&grid);
        assert!(part_numbers.iter().any(|pn| pn.value == 467));
        assert!(part_numbers.iter().any(|pn| pn.value == 598));
        assert_eq!(4361, sum_part_numbers(&grid));
        assert_eq!(467835, sum_gear_ratios(&grid));
    }

    #[test]
    fn part_one() {
        assert_eq!(535351, sum_part_numbers(&parse(INPUT)));
    }

    #[test]
    fn part_two() {
        assert_eq!(87287096, sum_gear_ratios(&parse(INPUT)));
    }
}
