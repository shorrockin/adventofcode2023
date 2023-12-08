use math::lcm;
use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    count_steps_until(&Instructions::new(input), "AAA", "ZZZ")
}

pub fn part_two(input: &str) -> usize {
    let instructions = Instructions::new(input);
    instructions
        .steps
        .iter()
        .filter(|(label, _)| label.ends_with('A'))
        .map(|(label, _)| count_steps_until(&instructions, label, "Z"))
        .fold(1, lcm)
}

fn count_steps_until(instructions: &Instructions, starting_step: &str, finish_step: &str) -> usize {
    let mut current_step = starting_step;
    let mut step_count = 0;

    loop {
        if current_step.ends_with(finish_step) {
            return step_count;
        }

        let index = step_count % instructions.instructions.len();
        let direction = &instructions.instructions[index];

        current_step = match direction {
            true => &instructions.steps[current_step].left,
            false => &instructions.steps[current_step].right,
        };

        step_count += 1;
    }
}

struct Instructions {
    instructions: Vec<bool>,
    steps: HashMap<String, Step>,
}
impl Instructions {
    fn new(input: &str) -> Instructions {
        let (instructions, steps) = input.split_once("\n\n").unwrap();

        Instructions {
            instructions: instructions.chars().map(|c| c == 'L').collect(),
            steps: steps
                .lines()
                .map(Step::new)
                .map(|s| (s.label.to_string(), s))
                .collect(),
        }
    }
}

struct Step {
    label: String,
    left: String,
    right: String,
}
impl Step {
    fn new(input: &str) -> Step {
        let (label, rest) = input.split_once(" = (").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();

        Step {
            label: label.to_string(),
            left: left.to_string(),
            right: right.trim_end_matches(')').to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_ONE: &str = include_str!("input.example.one.txt");
    static EXAMPLE_TWO: &str = include_str!("input.example.two.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(2, part_one(EXAMPLE_ONE));
        assert_eq!(20093, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(6, part_two(EXAMPLE_TWO));
        assert_eq!(22103062509257, part_two(INPUT));
    }
}
