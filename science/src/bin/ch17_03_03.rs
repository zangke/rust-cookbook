//! Filename: ch17_03_03.rs
//! Description: 复数的数学函数
//! Date: 2024/06/19 08:48:12

use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let x = Complex::new(0.0, 2.0*PI);

    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
