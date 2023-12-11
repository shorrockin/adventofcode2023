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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let input = "123\n456\n789";
        let expected = "741\n852\n963";
        assert_eq!(expected, rotate_right(input));
    }
}
