use std::ops::Range;

/// Given a range, check how many inputs could be valid passwords.
/// Password rules:
/// - Must be 6 digits, within the range given
/// - At least two adjacent digits are the same
/// - Digits are in ascending order
pub fn valid_inputs(inputs: Range<u32>) -> u32 {
    let mut valid = 0;

    for i in inputs {
        let as_str = i.to_string().chars().collect::<Vec<char>>();
        if as_str.len() != 6 {
            continue;
        }
        // Any digits not in order? Not valid
        if !as_str.windows(2).all(|d| d[0] <= d[1]) {
            continue;
        }
        // If no adjacent digits match
        if !as_str.windows(2).any(|d| d[0] == d[1]) {
            continue;
        }
        println!("Valid input: {}", i);
        valid += 1;
    }
    valid
}
