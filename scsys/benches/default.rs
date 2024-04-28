/*
    Appellation: default <bench>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(bench)]
extern crate test;

use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;

// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fib::fibonacci).collect::<Vec<_>>())
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| fib::Fibonacci::seq().take(BENCH_SIZE).collect::<Vec<_>>())
}

pub mod fib {

    // recursive fibonacci
    fn fibonacci(n: usize) -> u32 {
        if n < 2 {
            1
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    // iterative fibonacci
    pub struct Fibonacci {
        pub curr: u32,
        next: u32,
    }

    impl Fibonacci {
        pub fn new(curr: u32, next: u32) -> Self {
            Self { curr, next }
        }

        pub fn seq() -> Self {
            Self::new(1, 1)
        }

        pub fn value(&self) -> u32 {
            self.curr
        }
    }

    impl Default for Fibonacci {
        fn default() -> Self {
            Self { curr: 1, next: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            use std::mem::replace;

            let new_next = self.curr + self.next;
            let new_curr = replace(&mut self.next, new_next);

            Some(replace(&mut self.curr, new_curr))
        }
    }
}
