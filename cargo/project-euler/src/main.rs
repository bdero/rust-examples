#![feature(core)]
use std::iter::AdditiveIterator;

fn main() {
    println!("Please run the test suite to use; i.e. `cargo test`");
}

fn problem1() -> u32 {
    (0u32..1000)
        .filter(|n| n%3 == 0 || n%5 == 0)
        .sum()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(), 233168);
}
