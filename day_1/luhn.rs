// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // Ignore all spaces
    let num: String = cc_number
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .collect::<String>();

    // Reject number with less than two digits
    if num.len() < 2 {
        return false;
    }

    // Reject number with non-digit characters
    if let Err(_) = num.parse::<u64>() {
        return false;
    }

    // Moving from right to left, double every second digit
    let doubled = num
        .clone() // TODO: we should avoid cloning here
        .chars()
        .enumerate()
        .filter_map(|(i, n)| {
            if i % 2 == 0 {
                return None;
            };
            let d: u32 = n.to_digit(10).unwrap() * 2;
            if d < 10 {
                Some(d)
            } else {
                // sum the digits if the result is greater than 9
                let sd = d
                    .to_string()
                    .chars()
                    .into_iter()
                    .map(|n| n.to_digit(10).unwrap())
                    .sum();
                Some(sd)
            }
        })
        .sum::<u32>();

    let undoubled = num
        .chars()
        .enumerate()
        .filter_map(|(i, n)| {
            if i % 2 != 0 {
                return None;
            };
            return Some(n.to_digit(10).unwrap());
        })
        .sum::<u32>();

    (doubled + undoubled).to_string().chars().last().unwrap() == '0'
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
