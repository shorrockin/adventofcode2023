use crate::coordinate::Coordinate;
use crate::coordinate::Offset;
use std::collections::HashMap;

pub struct Grid {
    pub points: HashMap<Coordinate, char>,
}

impl std::str::FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                points.insert(Coordinate(x as i32, y as i32), c);
            }
        }
        Ok(Grid { points })
    }
}

impl Grid {
    pub fn get(&self, coord: Coordinate) -> Option<&char> {
        self.points.get(&coord)
    }

    pub fn get_offset(&self, coord: Coordinate, offset: Offset) -> Option<&char> {
        self.points.get(&(coord + offset))
    }

    pub fn get_all(&self, coords: Vec<Coordinate>) -> Vec<&char> {
        coords
            .iter()
            .map(|coord| self.get(*coord).unwrap())
            .collect()
    }

    pub fn is_empty(&self, coord: Coordinate) -> bool {
        self.get(coord).is_none()
    }

    pub fn is_empty_offset(&self, coord: Coordinate, offset: Offset) -> bool {
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
}
