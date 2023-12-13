use std::collections::HashMap;
use std::iter::repeat;

pub fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Row::new(l, 1))
        .map(|mut row| row.arrangements())
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Row::new(l, 5))
        .map(|mut row| row.arrangements())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
    pattern: Vec<Parts>,
    sections: Vec<usize>,
    memoized: HashMap<(usize, usize), i64>,
}
impl Row {
    fn new(line: &str, num_folds: usize) -> Row {
        let (pattern_base, sections_base) = line.split_once(' ').unwrap();

        let pattern = repeat(pattern_base)
            .take(num_folds)
            .collect::<Vec<_>>()
            .join("?");

        let sections = repeat(sections_base)
            .take(num_folds)
            .collect::<Vec<_>>()
            .join(",");

        Row {
            pattern: pattern
                .chars()
                .map(|c| match c {
                    '.' => Parts::Empty,
                    '?' => Parts::Wildcard,
                    '#' => Parts::Number,
                    _ => panic!("Invalid character in pattern"),
                })
                .collect(),
            sections: sections
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
            memoized: HashMap::new(),
        }
    }

    fn arrangements(&mut self) -> i64 {
        (0..self.next_number(0))
            .map(|i| self.arrangements_for(0, i))
            .sum()
    }

    fn arrangements_for(&mut self, section_index: usize, pattern_index: usize) -> i64 {
        if let Some(arrangements) = self.memoized.get(&(section_index, pattern_index)) {
            return *arrangements;
        }

        let result = self._arrangements_for(section_index, pattern_index);
        self.memoized.insert((section_index, pattern_index), result);
        result
    }

    fn _arrangements_for(&mut self, section_index: usize, pattern_index: usize) -> i64 {
        if !self.can_insert(section_index, pattern_index) {
            return 0;
        }

        let section_size = self.sections[section_index];

        // last section, we can insert this one as long as there's no other forced
        // numbers remaining
        if section_index == self.sections.len() - 1 {
            match (self.pattern[pattern_index + section_size..])
                .iter()
                .find(|p| **p == Parts::Number)
            {
                Some(_) => return 0,
                None => return 1,
            }
        }

        // not the last section, sum all the ways we can insert the next section, we'll move
        // our cursor over one to account for the inserted section
        let start_index = pattern_index + section_size + 1;
        let end_index = self.next_number(start_index);
        (start_index..end_index)
            .map(|i| self.arrangements_for(section_index + 1, i))
            .sum()
    }

    // when searching we can only consider elements up to the next guaranteed number
    // position, this will return that, or the end of the pattern if there are no more
    fn next_number(&self, offset: usize) -> usize {
        match (offset..self.pattern.len()).find(|i| self.pattern[*i] == Parts::Number) {
            Some(i) => i + 1,
            None => self.pattern.len(),
        }
    }

    fn can_insert(&self, section_index: usize, pattern_index: usize) -> bool {
        let section_size = self.sections[section_index];
        let end_index = pattern_index + section_size;

        // if we exceed the end of the pattern
        if end_index > self.pattern.len() {
            return false;
        }

        // if the next element after the pattern has to be a number, conditional to make sure
        // we're not at the end of the pattern
        if let Some(Parts::Number) = self.pattern.get(end_index) {
            return false;
        }

        // otherwise, it depends of if are any forced empty spaces in the pattern
        !(pattern_index..end_index).any(|i| self.pattern[i] == Parts::Empty)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Parts {
    Empty,
    Wildcard,
    Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("input.example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(1, Row::new("???.### 1,1,3", 1).arrangements());
        assert_eq!(4, Row::new(".??..??...?##. 1,1,3", 1).arrangements());
        assert_eq!(1, Row::new("?#?#?#?#?#?#?#? 1,3,1,6", 1).arrangements());
        assert_eq!(1, Row::new("????.#...#... 4,1,1", 1).arrangements());
        assert_eq!(4, Row::new("????.######..#####. 1,6,5", 1).arrangements());
        assert_eq!(10, Row::new("?###???????? 3,2,1", 1).arrangements());
        assert_eq!(21, part_one(EXAMPLE));
        assert_eq!(7173, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, Row::new("???.### 1,1,3", 5).arrangements());
        assert_eq!(16384, Row::new(".??..??...?##. 1,1,3", 5).arrangements());
        assert_eq!(525152, part_two(EXAMPLE));
        assert_eq!(29826669191291, part_two(INPUT));
    }
}
