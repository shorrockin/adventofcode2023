pub fn part_one(input: &str) -> i32 {
    solve(input, false)
}

pub fn part_two(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, reverse: bool) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut numbers: Vec<_> = line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
            if reverse {
                numbers.reverse();
            }
            solve_next(numbers)
        })
        .sum()
}

fn solve_next(values: Vec<i32>) -> i32 {
    let last = *values.last().unwrap();
    let next: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();

    if next.iter().all(|v| *v == 0) {
        return last;
    }

    last + solve_next(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("input.example.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(114, part_one(EXAMPLE));
        assert_eq!(2075724761, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(2, part_two(EXAMPLE));
        assert_eq!(1072, part_two(INPUT));
    }
}
