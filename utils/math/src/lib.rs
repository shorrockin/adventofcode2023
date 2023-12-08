use std::cmp::{max, min};

pub fn lcm(left: usize, right: usize) -> usize {
    let max_val = max(left, right);
    let min_val = min(left, right);

    for i in 1..=min_val {
        let multiple = max_val * i;
        if multiple % min_val == 0 {
            return multiple;
        }
    }

    max_val * min_val
}
