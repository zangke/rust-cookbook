//! Filename: ch17_05_01.rs
//! Description: 大数
//! Date: 2024/06/19 08:51:52

use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..(x + 1) {
            factorial = factorial * i;
        }
        factorial
    } else {
        panic!("Failed to calculate factorial!");
    }
}

fn main() {
    println!("{}! equals {}", 100, factorial(100));
}
