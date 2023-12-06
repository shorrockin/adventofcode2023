pub struct Race {
    pub time: i64,
    pub distance: i64,
}
impl Race {
    pub fn new(time: i64, distance: i64) -> Race {
        Race { time, distance }
    }

    pub fn count_tactics(&self) -> i64 {
        (0..=self.time)
            .filter(|time_held| (time_held * (self.time - time_held)) > self.distance)
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            114400_i64,
            [
                Race::new(35, 212),
                Race::new(93, 2060),
                Race::new(73, 1201),
                Race::new(66, 1044),
            ]
            .iter()
            .map(|r| r.count_tactics())
            .product()
        );
        assert_eq!(
            21039729,
            Race::new(35937366, 212206012011044).count_tactics()
        );
    }
}
