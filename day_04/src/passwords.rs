use std::ops::Range;

use itertools::Itertools;

/// Given a range, check how many inputs could be valid passwords.
/// Password rules:
/// - Must be 6 digits, within the range given
/// - Digits are in ascending order
/// - At least two adjacent digits are the same
/// Part 2 additional rule:
/// - Matching digits cannot be part of a larger set
/// --> This means, `11` is valid, but `111` isn't.
pub fn valid_inputs(inputs: Range<u32>) -> u32 {
    let mut valid = 0;

    for input in inputs {
        let as_str = input.to_string();
        let as_vec = as_str.chars().collect::<Vec<char>>();
        if as_vec.len() != 6 {
            continue;
        }
        // Any digits not in order? Not valid
        if !as_vec.windows(2).all(|d| d[0] <= d[1]) {
            continue;
        }
        // If no adjacent digits match
        if !as_vec.windows(2).any(|d| d[0] == d[1]) {
            continue;
        }
        // Part 2 rule: Matching digit can't be part of a larger set.
        let mut in_str = &as_str[..];
        if !in_str
            .chars()
            .group_by(|&c| c)
            .into_iter()
            .map(|(_, r)| {
                let len: usize = r.map(|c| c.len_utf8()).sum();
                let (a, b) = in_str.split_at(len);
                in_str = b;
                a
            })
            .any(|g| g.len() == 2)
        {
            continue;
        }
        println!("Valid input: {}", input);
        valid += 1;
    }
    valid
}
