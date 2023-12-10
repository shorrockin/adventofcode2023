#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Range;

pub fn part_one(input: &str) -> i64 {
    let almanac = Almanac::new(input);
    almanac
        .seeds
        .iter()
        .map(|s| almanac.resolve(*s))
        .min()
        .unwrap()
}

pub fn part_two(input: &str) -> i64 {
    let almanac = Almanac::new(input);
    almanac
        .seeds
        .chunks(2)
        .map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .map(|s| almanac.resolve(s))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
    //almanac.seeds.chunks(2).map(|chunk| chunk[1]).sum()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}
impl Almanac {
    fn new(input: &str) -> Almanac {
        let (seeds, maps) = input.split_once("\n\n").unwrap();

        Almanac {
            seeds: seeds
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect(),
            maps: maps.split("\n\n").map(Map::new).collect(),
        }
    }

    fn resolve(&self, value: i64) -> i64 {
        self.maps.iter().fold(value, |v, map| map.resolve(v))
    }

    fn resolve_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        todo!()
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    conversions: Vec<Conversion>,
}
impl Map {
    fn new(input: &str) -> Map {
        let (name, conversions) = input.split_once('\n').unwrap();

        Map {
            name: name.split_once(' ').unwrap().0.to_string(),
            conversions: conversions
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(Conversion::new)
                .collect(),
        }
    }

    fn resolve(&self, value: i64) -> i64 {
        match self.conversions.iter().find(|c| c.range.contains(&value)) {
            Some(conversion) => conversion.modifier + value,
            None => value,
        }
    }

    // when resolving a range, we have to convert it into chunks of ranges
    // depending on the conversions ranges. in the simplest case a single
    // conversion range will cover the range passed into, in which case we
    // can return a single range with the modifier applied. in most cases we
    // will have to split on the boundaries of the conversion ranges.
    fn resolve_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        todo!();
        /*
        let mut unresolved = vec![range];
        let mut resolved = Vec::new();

        self.conversions.iter().for_each(|conversion| {
            unresolved.iter().for_each(|target| {
                // the relationship between an unresolved target and a conversion
                // can fall into one of the following three categories of resolution:
                // 1. the target is completely contained within the conversion
                // 2. the target is completely outside of the conversion
                // 3. the target is partially contained within the conversion
            })
        });

        resolved
        */
    }
}

#[derive(Debug)]
struct Conversion {
    range: Range<i64>,
    modifier: i64,
}
impl Conversion {
    fn new(input: &str) -> Conversion {
        let values: Vec<i64> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Conversion {
            range: values[1]..values[1] + values[2],
            modifier: values[0] - values[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    #[test]
    fn test_parsing() {
        let almanac = Almanac::new(EXAMPLE);
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.maps.len(), 7);

        assert_eq!(almanac.maps[0].name, "seed-to-soil");
        assert_eq!(almanac.maps[0].conversions.len(), 2);
        assert_eq!(81, almanac.maps[0].resolve(79));
        assert_eq!(81, almanac.maps[1].resolve(81));
        assert_eq!(74, almanac.maps[3].resolve(81));
        assert_eq!(82, almanac.resolve(79));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(35, part_one(EXAMPLE));
        assert_eq!(196167384, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(46, part_two(EXAMPLE));
        //assert_eq!(125742456, part_two(INPUT));
    }
}
