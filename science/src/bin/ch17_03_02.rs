//! Filename: ch17_03_02.rs
//! Description: 复数相加
//! Date: 2024/06/19 08:47:31

fn main() {
    let complex_num1 = num::complex::Complex::new(10.0, 20.0); // 必须为浮点数
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}
