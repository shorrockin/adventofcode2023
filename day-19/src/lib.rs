use std::collections::HashMap;
use std::ops::Range;
use std::option::Option;
use strings::split_last;

const STARTING_WORKFLOW: &str = "in";

pub fn part_one(input: &str) -> i32 {
    let (workflows, parts) = parse(input);
    parts
        .iter()
        .filter(|part| is_accepted(part, workflows.get(STARTING_WORKFLOW).unwrap(), &workflows))
        .map(|part| part.rating())
        .sum()
}

fn is_accepted(part: &Part, workflow: &Workflow, workflows: &HashMap<String, Workflow>) -> bool {
    match workflow.apply(part) {
        Outcome::Accept => true,
        Outcome::Reject => false,
        Outcome::Workflow(id) => is_accepted(part, workflows.get(&id).unwrap(), workflows),
    }
}

pub fn part_two(input: &str) -> i64 {
    let (workflows, _) = parse(input);

    let ranges = success_ranges(
        &Ranges::default(),
        workflows.get(STARTING_WORKFLOW).unwrap(),
        0,
        &workflows,
    );

    ranges
        .iter()
        .map(|ranges| ranges.distinct_combinations())
        .sum()
}

fn success_ranges(
    ranges: &Ranges,
    workflow: &Workflow,
    rule_idx: usize,
    workflows: &HashMap<String, Workflow>,
) -> Vec<Ranges> {
    match workflow.rules.get(rule_idx) {
        Some(rule) => match rule.test_range(ranges) {
            Some((true_range, false_range)) => {
                let true_path = match &rule.outcome {
                    Outcome::Accept => vec![true_range],
                    Outcome::Reject => vec![],
                    Outcome::Workflow(id) => {
                        success_ranges(&true_range, workflows.get(id).unwrap(), 0, workflows)
                    }
                };
                let false_path = success_ranges(&false_range, workflow, rule_idx + 1, workflows);
                true_path.into_iter().chain(false_path).collect()
            }
            None => vec![],
        },
        None => match &workflow.fallback {
            Outcome::Accept => vec![ranges.clone()],
            Outcome::Reject => vec![],
            Outcome::Workflow(id) => {
                success_ranges(ranges, workflows.get(id).unwrap(), 0, workflows)
            }
        },
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflow_str, parts_str) = input.split_once("\n\n").unwrap();
    let parts: Vec<_> = parts_str.lines().map(Part::parse).collect();
    let workflows: HashMap<String, Workflow> = workflow_str
        .lines()
        .map(Workflow::parse)
        .map(|workflow| (workflow.id.clone(), workflow))
        .collect();

    (workflows, parts)
}

#[derive(Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
    fallback: Outcome,
}
impl Workflow {
    fn parse(input: &str) -> Self {
        let (id, rest) = input.split_once('{').unwrap();
        let (rules_str, outcome_str) = split_last(rest.strip_suffix('}').unwrap(), ',');
        let rules: Vec<_> = rules_str.split(',').map(Rule::parse).collect();

        Workflow {
            id: id.to_string(),
            rules,
            fallback: Outcome::parse(outcome_str),
        }
    }

    fn apply(&self, part: &Part) -> Outcome {
        match self.rules.iter().find(|rule| rule.test_part(part)) {
            Some(rule) => rule.outcome.clone(),
            None => self.fallback.clone(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    variable: Variable,
    condition: Condition,
    value: i32,
    outcome: Outcome,
}
impl Rule {
    fn parse(input: &str) -> Self {
        let (rule_str, outcome) = input.split_once(':').unwrap();
        let (variable, value) = rule_str.split_once(|c| c == '>' || c == '<').unwrap();
        Rule {
            variable: Variable::parse(variable),
            condition: Condition::parse(input),
            value: value.parse().unwrap(),
            outcome: Outcome::parse(outcome),
        }
    }

    fn test_part(&self, part: &Part) -> bool {
        let part_value = self.variable.get(part);
        match &self.condition {
            Condition::GreaterThan => part_value > self.value,
            Condition::LessThan => part_value < self.value,
        }
    }

    // given some range values, and this rule, we will split the range into two
    // branches, once success branch for this rule and one failure branch based
    // on the state of the rule
    fn test_range(&self, ranges: &Ranges) -> Option<(Ranges, Ranges)> {
        let existing = match self.variable {
            Variable::X => &ranges.x,
            Variable::M => &ranges.m,
            Variable::A => &ranges.a,
            Variable::S => &ranges.s,
        };

        if !existing.contains(&self.value) {
            return None;
        }

        Some((
            ranges.clone_with(&self.variable, self.split_ranges(existing, true)),
            ranges.clone_with(&self.variable, self.split_ranges(existing, false)),
        ))
    }

    fn split_ranges(&self, current: &Range<i32>, success: bool) -> Range<i32> {
        match (success, &self.condition) {
            // example: a>1716: [false: 1..1717, true: 1717..4001]
            (false, Condition::GreaterThan) => current.start..self.value + 1,
            (true, Condition::GreaterThan) => self.value + 1..current.end,
            // example: a<2006: [true: 1..2006, false: 2006..4001]
            (true, Condition::LessThan) => current.start..self.value,
            (false, Condition::LessThan) => self.value..current.end,
        }
    }
}

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}
impl Variable {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Variable::X,
            "m" => Variable::M,
            "a" => Variable::A,
            "s" => Variable::S,
            v => panic!("invalid variable: {}", v),
        }
    }

    fn get(&self, part: &Part) -> i32 {
        match self {
            Variable::X => part.x,
            Variable::M => part.m,
            Variable::A => part.a,
            Variable::S => part.s,
        }
    }
}

#[derive(Debug)]
enum Condition {
    GreaterThan,
    LessThan,
}
impl Condition {
    fn parse(input: &str) -> Self {
        match input.contains('>') {
            true => Condition::GreaterThan,
            false => Condition::LessThan,
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Accept,
    Reject,
    Workflow(String),
}
impl Outcome {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            _ => Outcome::Workflow(input.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Part {
    fn parse(input: &str) -> Self {
        let values: Vec<i32> = input
            .split(|c| c == '=' || c == ',' || c == '}')
            .flat_map(|s| s.parse())
            .collect();
        assert!(values.len() == 4);

        Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }

    fn rating(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

// non-overlapping set of ranges that apply to each variable rule
#[derive(Debug, Clone)]
struct Ranges {
    x: Range<i32>,
    m: Range<i32>,
    a: Range<i32>,
    s: Range<i32>,
}
impl Ranges {
    fn default() -> Self {
        Ranges {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn new(x: Range<i32>, m: Range<i32>, a: Range<i32>, s: Range<i32>) -> Self {
        Ranges { x, m, a, s }
    }

    fn clone_with(&self, variable: &Variable, ranges: Range<i32>) -> Self {
        match variable {
            Variable::X => Ranges::new(ranges, self.m.clone(), self.a.clone(), self.s.clone()),
            Variable::M => Ranges::new(self.x.clone(), ranges, self.a.clone(), self.s.clone()),
            Variable::A => Ranges::new(self.x.clone(), self.m.clone(), ranges, self.s.clone()),
            Variable::S => Ranges::new(self.x.clone(), self.m.clone(), self.a.clone(), ranges),
        }
    }

    fn distinct_combinations(&self) -> i64 {
        self.x.len() as i64 * self.m.len() as i64 * self.a.len() as i64 * self.s.len() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(19114, part_one(EXAMPLE));
        assert_eq!(495298, part_one(INPUT));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(167409079868000, part_two(EXAMPLE));
        assert_eq!(132186256794011, part_two(INPUT));
    }
}
