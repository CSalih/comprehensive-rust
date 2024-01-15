/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 0;
    while n > 1 {
        length += 1;
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }
    }

    return length;
}

fn main() {
    println!("Collatz length of 3 is {}", collatz_length(3))
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(1), 0);
    assert_eq!(collatz_length(3), 7);
}
