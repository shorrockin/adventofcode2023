use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::RangeInclusive;
use strings::AlphabeticCounter;

pub fn part_one(input: &str) -> usize {
    solve(input, true)
}

pub fn part_two(input: &str) -> usize {
    solve(input, false)
}

fn solve(input: &str, part_one: bool) -> usize {
    let mut current_z = 1;
    let mut counter = AlphabeticCounter::new();
    let mut supporting: HashMap<String, Vec<String>> = HashMap::new();
    let mut supported_by: HashMap<String, Vec<String>> = HashMap::new();
    let (mut settled, mut unsettled): (Vec<Brick>, Vec<_>) = input
        .lines()
        .map(|l| Brick::parse(&counter.next().unwrap(), l))
        .partition(|brick| brick.on_ground());

    while !unsettled.is_empty() {
        current_z += 1;

        let (to_settle, still_unsettled): (Vec<_>, Vec<_>) = unsettled
            .into_iter()
            .partition(|brick| brick.at_level(current_z));

        unsettled = still_unsettled;

        // now we will look for the bricks for which this brick can settle upon
        // starting on the current z, move down a level, look for candidates, then
        // once we find > 0 candidates settle the block and record who it is able t
        // settle on
        for mut settling in to_settle.into_iter() {
            loop {
                let settling_on: Vec<_> = settled
                    .iter()
                    .filter(|brick| settling.can_settle_on(brick))
                    .collect();

                if !settling_on.is_empty() || settling.on_ground() {
                    for brick in settling_on {
                        supporting
                            .entry(brick.id.clone())
                            .or_default()
                            .push(settling.id.clone());
                        supported_by
                            .entry(settling.id.clone())
                            .or_default()
                            .push(brick.id.clone());
                    }

                    settled.push(settling.clone());

                    break;
                }

                settling = settling.fall();
            }
        }
    }

    match part_one {
        true => settled
            .iter()
            .filter(|brick| match supporting.get(&brick.id) {
                None => true,
                Some(s) => s.iter().all(|id| supported_by.get(id).unwrap().len() > 1),
            })
            .count(),
        false => settled
            .iter()
            .map(|brick| disolve(brick, &supported_by))
            .sum(),
    }
}

fn disolve(brick: &Brick, supported_by: &HashMap<String, Vec<String>>) -> usize {
    let mut supported = supported_by.clone();
    let mut disolving = VecDeque::new();

    disolving.push_front(brick.id.clone());
    while let Some(id) = disolving.pop_front() {
        for (k, v) in supported.iter_mut() {
            if v.contains(&id) {
                v.retain(|x| x != &id);
                if v.is_empty() {
                    disolving.push_front(k.clone());
                }
            }
        }
    }

    supported.iter().filter(|(_, v)| v.is_empty()).count()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    id: String,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}
impl Brick {
    fn parse(id: &str, input: &str) -> Self {
        let (from_str, to_str) = input.split_once('~').unwrap();
        let from: Vec<_> = from_str.split(',').collect();
        let to: Vec<_> = to_str.split(',').collect();

        let brick = Brick {
            id: id.to_string(),
            x: (from[0].parse().unwrap()..=to[0].parse().unwrap()),
            y: (from[1].parse().unwrap()..=to[1].parse().unwrap()),
            z: (from[2].parse().unwrap()..=to[2].parse().unwrap()),
        };

        // these should always be true, assumptions in logic baked in
        assert!(brick.x.start() <= brick.x.end());
        assert!(brick.y.start() <= brick.y.end());
        assert!(brick.z.start() <= brick.z.end());

        brick
    }

    fn on_ground(&self) -> bool {
        self.at_level(1)
    }

    fn at_level(&self, level: usize) -> bool {
        self.z.contains(&level)
    }

    fn fall(&self) -> Brick {
        if self.z.contains(&1) {
            panic!("cannot fall any further");
        }

        Brick {
            id: self.id.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            z: (self.z.start() - 1)..=(self.z.end() - 1),
        }
    }

    fn can_settle_on(&self, target: &Brick) -> bool {
        target.z.contains(&(self.z.start() - 1))
            && self.x.clone().any(|x| target.x.contains(&x))
            && self.y.clone().any(|y| target.y.contains(&y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(5, part_one(EXAMPLE));
        assert_eq!(519, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(7, part_two(EXAMPLE));
        assert_eq!(109531, part_two(INPUT));
    }
}
