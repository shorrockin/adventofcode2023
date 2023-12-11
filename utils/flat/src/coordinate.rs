#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub i32, pub i32);

impl core::ops::Add<Offset> for Coordinate {
    type Output = Coordinate;
    fn add(self, direction: Offset) -> Coordinate {
        Coordinate(self.0 + direction.0, self.1 + direction.1)
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Coordinate({},{})", self.0, self.1))
    }
}

impl std::str::FromStr for Coordinate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Coordinate(x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl Coordinate {
    pub fn distance(&self, other: &Coordinate) -> u32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u32
    }

    pub fn cardinals(&self) -> Vec<Coordinate> {
        vec![
            *self + offsets::NORTH,
            *self + offsets::SOUTH,
            *self + offsets::EAST,
            *self + offsets::WEST,
        ]
    }

    pub fn intercardinals(&self) -> Vec<Coordinate> {
        vec![
            *self + offsets::NORTH,
            *self + offsets::NORTH_EAST,
            *self + offsets::NORTH_WEST,
            *self + offsets::SOUTH,
            *self + offsets::SOUTH_EAST,
            *self + offsets::SOUTH_WEST,
            *self + offsets::EAST,
            *self + offsets::WEST,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Offset(pub i32, pub i32);

impl Offset {
    pub fn invert(&self) -> Offset {
        Offset(-self.0, -self.1)
    }
}

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Offset({},{})", self.0, self.1))
    }
}

impl std::str::FromStr for Offset {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Offset(x.parse().unwrap(), y.parse().unwrap()))
    }
}

pub mod offsets {
    use super::*;
    pub static NORTH: Offset = Offset(0, -1);
    pub static NORTH_WEST: Offset = Offset(-1, -1);
    pub static NORTH_EAST: Offset = Offset(1, -1);
    pub static SOUTH: Offset = Offset(0, 1);
    pub static SOUTH_WEST: Offset = Offset(-1, 1);
    pub static SOUTH_EAST: Offset = Offset(1, 1);
    pub static EAST: Offset = Offset(1, 0);
    pub static WEST: Offset = Offset(-1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_addition() {
        let c = Coordinate(1, 1);
        let n = c + offsets::NORTH;
        assert_eq!(n, Coordinate(1, 0));
    }
}
