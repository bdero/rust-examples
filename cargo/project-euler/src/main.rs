#![feature(core)]
use std::iter::AdditiveIterator;


fn main() {
    println!("Please run the test suite to use; i.e. `cargo test`");
}


// PROBLEM 1: Multiples of 3 and 5
// https://projecteuler.net/problem=1

fn problem1() -> u32 {
    (0u32..1000)
        .filter(|n| n%3 == 0 || n%5 == 0)
        .sum()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(), 233168);
}


// PROBLEM 2: Even Fibonacci numbers
// https://projecteuler.net/problem=2

fn problem2() -> u32 {
    (Fibonacci { current: 1, next: 1 })
        .take_while(|&n| n < 4000000u32)
        .filter(|n| n%2 == 0)
        .sum()
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(), 4613732);
}

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.current + self.next;

        self.current = self.next;
        self.next = next;

        Some(self.current)
    }
}
