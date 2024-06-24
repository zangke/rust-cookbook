//! Filename: ch17_03_01.rs
//! Description: 创建复数
//! Date: 2024/06/19 08:46:16

fn main() {
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}
