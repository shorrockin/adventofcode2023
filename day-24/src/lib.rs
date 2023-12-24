use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

pub fn part_one(input: &str, min: f64, max: f64) -> usize {
    let coords = parse(input);

    let mut permutations = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            permutations.push((coords[i], coords[j]));
        }
    }

    permutations
        .iter()
        .filter_map(|(a, b)| intersection(a, b))
        .filter(|(x, y)| *x >= min && *x <= max && *y >= min && *y <= max)
        .count()
}

pub fn part_two(input: &str) -> i64 {
    let coords = parse(input);
    let context = Context::new(&Config::new());
    let solver = Solver::new(&context);

    let x = Int::new_const(&context, "x");
    let y = Int::new_const(&context, "y");
    let z = Int::new_const(&context, "z");
    let vx = Int::new_const(&context, "vx");
    let vy = Int::new_const(&context, "vy");
    let vz = Int::new_const(&context, "vz");

    // only need 3 coordinates to prove this out
    for (i, coord) in coords.iter().take(3).enumerate() {
        let a = Int::from_i64(&context, coord.x);
        let b = Int::from_i64(&context, coord.y);
        let c = Int::from_i64(&context, coord.z);
        let va = Int::from_i64(&context, coord.vx);
        let vb = Int::from_i64(&context, coord.vy);
        let vc = Int::from_i64(&context, coord.vz);

        let t = Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

    if solver.check() != SatResult::Sat {
        panic!("failed to calculate, so solution found");
    }

    let x = match solver.get_model() {
        Some(m) => m.eval(&(x + y + z), true).unwrap().as_i64().unwrap(),
        None => panic!("failed to calculate, could not get model"),
    };
    x // needed binding for borrow checker ¯\_(ツ)_/¯
}

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|line| {
            let (coords_str, velocities_str) = line.split_once(" @ ").unwrap();
            let coords: Vec<_> = coords_str.split(", ").map(|s| s.parse().unwrap()).collect();
            let velocities: Vec<_> = velocities_str
                .split(", ")
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Coordinate {
                x: coords[0],
                y: coords[1],
                z: coords[2],
                vx: velocities[0],
                vy: velocities[1],
                vz: velocities[2],
            }
        })
        .collect()
}

fn intersection(a: &Coordinate, b: &Coordinate) -> Option<(f64, f64)> {
    let det = (a.vx * b.vy - b.vx * a.vy) as f64;

    if det == 0. {
        None // lines are parallel, no intersection
    } else {
        let xdiff: f64 = a.x as f64 - b.x as f64;
        let ydiff: f64 = a.y as f64 - b.y as f64;
        let t: f64 = (b.vx as f64 * ydiff - b.vy as f64 * xdiff) / det;
        let u: f64 = (a.vx as f64 * ydiff - a.vy as f64 * xdiff) / det;

        if t >= 0. && u >= 0. {
            let x = a.x as f64 + t * a.vx as f64;
            let y = a.y as f64 + t * a.vy as f64;
            Some((x, y))
        } else {
            None // lines are not intersect in the direction of the vectors
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(EXAMPLE, 7.0, 27.0));
        assert_eq!(15558, part_one(INPUT, 200000000000000.0, 400000000000000.0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(47, part_two(EXAMPLE));
        // too slow for tests
        //assert_eq!(765636044333842, part_two(INPUT));
    }
}
