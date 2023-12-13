use crate::coordinate::Coordinate;
use crate::coordinate::Offset;
use std::collections::HashMap;

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

    pub fn is_empty(&self, coord: &Coordinate) -> bool {
        self.get(coord).is_none()
    }

    pub fn is_empty_offset(&self, coord: &Coordinate, offset: Offset) -> bool {
        self.get_offset(coord, offset).is_none()
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

    pub fn rotate_right(&self) -> Grid {
        let mut new_grid = Grid::new();
        for (coord, c) in self.points.iter() {
            let new_coord = Coordinate(coord.1, coord.0);
            new_grid.points.insert(new_coord, *c);
            new_grid.bounds.update(new_coord);
        }
        new_grid
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
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
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            x: BoundValue { min: 0, max: 0 },
            y: BoundValue { min: 0, max: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundValue {
    pub min: i32,
    pub max: i32,
}
