use math::lcm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> i32 {
    let modules = parse(input);
    let mut processing: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        processing.push_back(("button".to_string(), "roadcaster".to_string(), false));
        while let Some((from, to, signal)) = processing.pop_front() {
            match signal {
                true => high_count += 1,
                false => low_count += 1,
            }

            if let Some(refcell) = modules.get(&to) {
                let mut module = refcell.borrow_mut();

                if let Some(result) = module.pulse(&from, signal) {
                    for destination in module.destinations.iter() {
                        processing.push_back((to.clone(), destination.clone(), result));
                    }
                }
            }
        }
    }
    low_count * high_count
}

pub fn part_two(input: &str, rx_sender: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(_, module)| {
            module
                .borrow()
                .destinations
                .contains(&rx_sender.to_string())
        })
        .map(|(id, _)| id.to_string())
        .map(|id| detect_signal(&parse(input), &id, rx_sender))
        .fold(1, lcm)
}

fn detect_signal(
    modules: &HashMap<String, RefCell<Module>>,
    detect_from: &str,
    detect_to: &str,
) -> usize {
    let mut processing: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut count = 0;

    loop {
        count += 1;
        processing.push_back(("button".to_string(), "roadcaster".to_string(), false));
        while let Some((from, to, signal)) = processing.pop_front() {
            if &to == detect_to && &from == detect_from && signal {
                return count;
            }

            if let Some(refcell) = modules.get(&to) {
                let mut module = refcell.borrow_mut();

                if let Some(result) = module.pulse(&from, signal) {
                    for destination in module.destinations.iter() {
                        processing.push_back((to.clone(), destination.clone(), result));
                    }
                }
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, RefCell<Module>> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.id.to_string(), m))
        .collect();

    let destinations: HashMap<_, _> =
        modules
            .iter()
            .fold(HashMap::new(), |mut acc, (id, module)| {
                for destination in &module.destinations {
                    acc.entry(destination.to_string())
                        .or_insert_with(Vec::new)
                        .push(id.to_string());
                }
                acc
            });

    for (id, module) in &mut modules {
        match module.state {
            State::ConjunctionUninitialized => {
                module.state = State::Conjunction(
                    destinations
                        .get(id)
                        .unwrap()
                        .iter()
                        .map(|dest| (dest.to_string(), false))
                        .collect(),
                );
            }
            _ => (),
        }
    }

    modules
        .into_iter()
        .map(|(id, module)| (id.to_string(), RefCell::new(module)))
        .collect()
}

#[derive(Debug, Clone)]
struct Module {
    id: String,
    state: State,
    destinations: Vec<String>,
}
impl Module {
    fn parse(input: &str) -> Self {
        let (type_id, destinations) = input.split_once(" -> ").unwrap();
        let (type_val, id) = type_id.split_at(1);
        let module_type = match type_val {
            "b" => State::Broadcaster,
            "%" => State::FlipFlop(false),
            "&" => State::ConjunctionUninitialized,
            _ => panic!("unknown module type"),
        };

        Self {
            id: id.to_string(),
            state: module_type,
            destinations: destinations.split(", ").map(|s| s.to_string()).collect(),
        }
    }

    fn pulse(&mut self, from: &str, signal: bool) -> Option<bool> {
        self.state.pulse(from, signal)
    }
}

#[derive(Debug, Clone)]
enum State {
    Broadcaster,
    FlipFlop(bool),
    ConjunctionUninitialized,
    Conjunction(HashMap<String, bool>),
}
impl State {
    fn pulse(&mut self, from: &str, signal: bool) -> Option<bool> {
        match (self, signal) {
            (State::Broadcaster, _) => Some(signal),
            (State::FlipFlop(_), true) => None,
            (State::FlipFlop(value), false) => {
                *value = !*value;
                Some(*value)
            }
            (State::Conjunction(states), _) => {
                states.insert(from.to_string(), signal);
                Some(!states.values().all(|v| *v))
            }
            _ => panic!("unable to handle signal for state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = include_str!("input.example.one.txt");
    const EXAMPLE_TWO: &str = include_str!("input.example.two.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(32000000, part_one(EXAMPLE_ONE), "example one");
        assert_eq!(11687500, part_one(EXAMPLE_TWO), "example two");
        assert_eq!(807069600, part_one(INPUT), "input");
    }

    #[test]
    fn part_two_works() {
        assert_eq!(221453937522197, part_two(INPUT, &"hf"), "input");
    }
}
