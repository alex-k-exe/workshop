use std::collections::HashSet;

pub fn is_n_zeroable(n: i32) -> bool {
    /*
    Given an integer n, determine whether its digits can be simplified to 0 with the four fundemental operations - add, subtract, multiply, and divide
    Parentheses are allowed
    Cannot repeat digits

    365 -> no
    1234 -> (1 + 2 - 3) * 4 = 0 -> yes
    1247 -> 2 * 4 - 7 - 1 = 0 -> yes
    5789
        578, 579
    4789 -> 9 - 7 - 8/4
    */

    if n > 9999 {
        return true;
    }

    // split n into its digits
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // check if n has a 0
    if digits.iter().any(|&digit| digit == 0) {
        return true;
    }

    // check if n has duplicate digits
    let unique_digits: HashSet<_> = digits.iter().cloned().collect();
    if digits.len() != unique_digits.len() {
        return true;
    }

    return recursive_combinations(digits);
}

fn recursive_combinations(digits: Vec<u32>) -> bool {
    if digits.len() <= 3 {
        return true;
    } else {
        return false;
    }
}
