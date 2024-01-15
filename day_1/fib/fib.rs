fn fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n));
}

#[test]
fn test_fib() {
    assert_eq!(fib(0), 1);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(30), 832040);
}
