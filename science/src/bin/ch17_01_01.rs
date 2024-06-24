//! Filename: ch17_01_01.rs
//! Description: 矩阵相加
//! Date: 2024/06/18 08:37:13

use ndarray::arr2;

fn main() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}
