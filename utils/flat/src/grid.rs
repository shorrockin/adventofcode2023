use crate::coordinate::Coordinate;
use crate::coordinate::Offset;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

pub struct Grid {
    pub points: HashMap<Coordinate, char>,
    pub bounds: Bounds,
}

impl std::str::FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();
        let mut bounds = Bounds::default();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coordinate(x as i32, y as i32);
                points.insert(coord, c);
                bounds.update(coord);
            }
        }
        Ok(Grid { points, bounds })
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            points: HashMap::new(),
            bounds: Bounds::default(),
        }
    }

    pub fn parse(input: &str) -> Grid {
        Grid::from_str(input).unwrap()
    }

    pub fn get(&self, coord: &Coordinate) -> Option<&char> {
        self.points.get(coord)
    }

    pub fn get_with_default(&self, coord: &Coordinate, default: char) -> char {
        match self.points.get(coord) {
            Some(c) => *c,
            None => default,
        }
    }

    pub fn get_offset(&self, coord: &Coordinate, offset: Offset) -> Option<&char> {
        self.points.get(&(*coord + offset))
    }

    pub fn get_all(&self, coords: Vec<Coordinate>) -> Vec<&char> {
        coords
            .iter()
            .map(|coord| self.get(coord).unwrap())
            .collect()
    }

    pub fn find(&self, char: &char) -> Option<Coordinate> {
        for (coord, c) in self.points.iter() {
            if c == char {
                return Some(*coord);
            }
        }
        None
    }

    pub fn find_all(&self, char: &char) -> Vec<Coordinate> {
        self.points
            .iter()
            .flat_map(|(coord, c)| match c == char {
                true => Some(*coord),
                false => None,
            })
            .collect()
    }

    pub fn insert(&mut self, coord: Coordinate, char: char) {
        self.points.insert(coord, char);
        self.bounds.update(coord);
    }

    pub fn move_point(&mut self, from: Coordinate, to: Coordinate) {
        if !self.points.contains_key(&from) {
            panic!(
                "Cannot move from {:?} to {:?}, no point at {:?}",
                from, to, from
            );
        }

        if self.points.contains_key(&to) {
            panic!(
                "Cannot move from {:?} to {:?}, point already exists at {:?}",
                from, to, to
            );
        }

        let c = self.points.remove(&from).unwrap();
        self.points.insert(to, c);
    }

    pub fn remove_char(&mut self, char: char) {
        let mut to_remove = Vec::new();
        for (coord, c) in self.points.iter() {
            if *c == char {
                to_remove.push(*coord);
            }
        }
        for coord in to_remove {
            self.points.remove(&coord);
        }
    }

    pub fn is_empty(&self, coord: &Coordinate) -> bool {
        self.get(coord).is_none()
    }

    pub fn is_some(&self, coord: &Coordinate) -> bool {
        self.get(coord).is_some()
    }

    pub fn is_empty_offset(&self, coord: &Coordinate, offset: Offset) -> bool {
        self.get_offset(coord, offset).is_none()
    }

    pub fn is_not_equal(&self, coord: &Coordinate, char: &char) -> bool {
        match self.get(coord) {
            Some(c) => c != char,
            None => true,
        }
    }

    pub fn is_equal(&self, coord: &Coordinate, char: &char) -> bool {
        match self.get(coord) {
            Some(c) => c == char,
            None => false,
        }
    }

    pub fn in_bounds(&self, coord: &Coordinate) -> bool {
        coord.0 >= self.bounds.x.min
            && coord.0 <= self.bounds.x.max
            && coord.1 >= self.bounds.y.min
            && coord.1 <= self.bounds.y.max
    }

    pub fn rotate_right(&self) -> Grid {
        let mut new_grid = Grid::new();
        for (coord, c) in self.points.iter() {
            let new_coord = Coordinate(coord.1, coord.0);
            new_grid.points.insert(new_coord, *c);
            new_grid.bounds.update(new_coord);
        }
        new_grid
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut hash_codes: Vec<u64> = self
            .points
            .iter()
            .map(|(key, value)| {
                let mut s = DefaultHasher::new();
                key.hash(&mut s);
                value.hash(&mut s);
                s.finish()
            })
            .collect();

        hash_codes.sort_unstable();

        for hash in &hash_codes {
            hash.hash(&mut hasher);
        }

        hasher.finish()
    }

    pub fn format_default(&self) -> String {
        self.format(GridFormatter::default())
    }

    pub fn format(&self, formatter: GridFormatter) -> String {
        (0..=self.bounds.y.max)
            .map(|y| {
                (0..=self.bounds.x.max)
                    .map(|x| match self.get(&Coordinate(x, y)) {
                        Some(c) => formatter
                            .replace
                            .get(c)
                            .unwrap_or(&c.to_string())
                            .to_owned(),
                        None => formatter.empty.to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GridFormatter {
    empty: char,
    replace: HashMap<char, String>,
}
impl GridFormatter {
    pub fn new(empty: char, replace: Vec<(char, String)>) -> GridFormatter {
        GridFormatter {
            empty,
            replace: replace.into_iter().collect(),
        }
    }
}
impl Default for GridFormatter {
    fn default() -> Self {
        GridFormatter {
            empty: ' ',
            replace: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bounds {
    pub x: BoundValue,
    pub y: BoundValue,
}
impl Bounds {
    fn update(&mut self, coord: Coordinate) {
        self.x.min = self.x.min.min(coord.0);
        self.x.max = self.x.max.max(coord.0);
        self.y.min = self.y.min.min(coord.1);
        self.y.max = self.y.max.max(coord.1);
    }

    pub fn max(&self) -> Coordinate {
        Coordinate(self.x.max, self.y.max)
    }

    pub fn min(&self) -> Coordinate {
        Coordinate(self.x.min, self.y.min)
    }

    pub fn width(&self) -> i32 {
        self.x.size()
    }

    pub fn height(&self) -> i32 {
        self.y.size()
    }

    // if our bounds are unbounded, thus repeat infinitely, then we need
    // to convert our unbounded coordinates to our local coordinate
    pub fn unbounded_to_local(&self, coord: &Coordinate) -> Coordinate {
        assert_eq!(0, self.x.min, "this doesn't work for non-zero min x bounds");
        assert_eq!(0, self.y.min, "this doesn't work for non-zero min y bounds");

        Coordinate(
            coord.0.rem_euclid(self.width()),
            coord.1.rem_euclid(self.height()),
        )
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            x: BoundValue { min: 0, max: 0 },
            y: BoundValue { min: 0, max: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BoundValue {
    pub min: i32,
    pub max: i32,
}
impl BoundValue {
    pub fn size(&self) -> i32 {
        // +1 because we're inclusive
        self.max - self.min + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unbounded_coordinates_can_be_translated() {
        let bounds = Bounds {
            x: BoundValue { min: 0, max: 5 },
            y: BoundValue { min: 0, max: 6 },
        };
        assert_eq!(6, bounds.width());
        assert_eq!(7, bounds.height());
        assert_eq!(
            Coordinate(1, 4),
            bounds.unbounded_to_local(&Coordinate(7, -3))
        );
        assert_eq!(
            Coordinate(5, 6),
            bounds.unbounded_to_local(&Coordinate(5, 6))
        );
    }
}
