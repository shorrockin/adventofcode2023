pub fn sum_values(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }
        if first.is_some() && last.is_some() {
            let joined = first.unwrap().to_string() + &last.unwrap().to_string();
            sum += joined.parse::<u32>().unwrap();
        }
    }
    sum
}

pub fn normalize_numbers(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");
    static EXAMPLE_ONE: &str = include_str!("input.example1.txt");
    static EXAMPLE_TWO: &str = include_str!("input.example2.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(sum_values(EXAMPLE_ONE), 142);
        assert_eq!(sum_values(INPUT), 53651);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(normalize_numbers("eightwothree"), "e8t2ot3e");
        assert_eq!(normalize_numbers("eighthree"), "e8t3e");
        assert_eq!(sum_values(&normalize_numbers(EXAMPLE_TWO)), 281);
        assert_eq!(sum_values(&normalize_numbers(INPUT)), 53894);
    }
}
