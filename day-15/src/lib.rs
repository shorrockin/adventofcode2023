use std::iter::repeat_with;

pub fn part_one(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

pub fn part_two(input: &str) -> usize {
    let mut boxes: Vec<_> = repeat_with(Box::default).take(256).collect();

    input.trim().split(',').for_each(|instruction| {
        let (label, fl) = instruction.split_once(|c| c == '-' || c == '=').unwrap();
        let box_idx = hash(label);

        match fl.is_empty() {
            true => boxes[box_idx].lenses.retain(|lens| lens.label != label),
            false => match boxes[box_idx]
                .lenses
                .iter_mut()
                .find(|lens| lens.label == label)
            {
                Some(lens) => lens.focal_length = fl.parse().unwrap(),
                None => {
                    boxes[box_idx].lenses.push(Lens {
                        label: label.to_string(),
                        focal_length: fl.parse().unwrap(),
                    });
                }
            },
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, light_box)| {
            light_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn hash(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[derive(Default)]
struct Box {
    lenses: Vec<Lens>,
}

struct Lens {
    label: String,
    focal_length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(1320, part_one(EXAMPLE));
        assert_eq!(501680, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(145, part_two(EXAMPLE));
        assert_eq!(241094, part_two(INPUT));
    }
}
