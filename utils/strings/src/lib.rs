pub fn rotate_right(input: &str) -> String {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut transformed = String::new();

    for i in 0..width {
        for j in (0..height).rev() {
            transformed.push(lines[j][i]);
        }

        if i < width - 1 {
            transformed.push('\n');
        }
    }

    transformed
}

pub fn split_last(input: &str, delimiter: char) -> (&str, &str) {
    match input.rfind(delimiter) {
        Some(index) => (&input[..index], &input[index + 1..]),
        None => (input, ""),
    }
}

pub struct AlphabeticCounter {
    next_char: u8,
    counter: usize,
}
impl AlphabeticCounter {
    pub fn new() -> AlphabeticCounter {
        AlphabeticCounter {
            next_char: b'A',
            counter: 0,
        }
    }
}
impl Default for AlphabeticCounter {
    fn default() -> Self {
        Self::new()
    }
}
impl Iterator for AlphabeticCounter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_char > b'Z' {
            self.next_char = b'A';
            self.counter += 1;
        }

        let mut str = String::from_utf8(vec![self.next_char]).unwrap();

        if self.counter > 0 {
            str.push_str(&self.counter.to_string());
        }

        self.next_char += 1;

        Some(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let input = "123\n456\n789";
        let expected = "741\n852\n963";
        assert_eq!(expected, rotate_right(input));
    }

    #[test]
    fn test_split_last() {
        assert_eq!(("abc,efg", "def"), split_last("abc,efg,def", ','));
    }
}
