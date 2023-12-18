use flat::coordinate::Coordinate;
use flat::coordinate::Direction::{East, North, South, West};
use flat::coordinate::Offset;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+)\s(\d+)\s\(#(\w{5})(\w{1})\)").unwrap();
}

pub fn part_one(input: &str) -> i64 {
    solve(
        input
            .lines()
            .map(|line| {
                let captures = RE.captures(line).unwrap();
                match &captures[1] {
                    "R" => East,
                    "D" => South,
                    "L" => West,
                    "U" => North,
                    _ => panic!("invalid direction"),
                }
                .value()
                .times(captures[2].parse().unwrap())
            })
            .collect(),
    )
}

pub fn part_two(input: &str) -> i64 {
    solve(
        input
            .lines()
            .map(|line| {
                let captures = RE.captures(line).unwrap();
                match &captures[4] {
                    "0" => East,
                    "1" => South,
                    "2" => West,
                    "3" => North,
                    _ => panic!("invalid direction"),
                }
                .value()
                .times(i32::from_str_radix(&captures[3], 16).unwrap())
            })
            .collect(),
    )
}

fn solve(offsets: Vec<Offset>) -> i64 {
    let mut perimiter = 0;
    let mut coords = vec![Coordinate(0, 0)];

    for offset in offsets {
        perimiter += offset.0.abs().max(offset.1.abs()) as i64;
        coords.push(*coords.last().unwrap() + offset);
    }

    // shoelace returns area for vertices located in the middle, but the
    // edges have 1 meter border so there's is an extra 1/2 meter (perim / 2)
    // needed to account. then for each corner the diff is 1/4 sqm, so we
    // need to +1 to account for this (full loop means that difference between
    // added and removed corners is always 4)
    shoelace(&coords) + perimiter / 2 + 1
}

fn shoelace(coords: &Vec<Coordinate>) -> i64 {
    let mut area: i64 = 0;
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        area += coords[i].0 as i64 * coords[j].1 as i64;
        area -= coords[i].1 as i64 * coords[j].0 as i64;
    }
    area.abs() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(62, part_one(EXAMPLE));
        assert_eq!(35401, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(952408144115, part_two(EXAMPLE));
        assert_eq!(48020869073824, part_two(INPUT));
    }
}
