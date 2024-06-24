//! Filename: ch04_02_01.rs
//! Description: 并行改变数组中元素
//! Date: 2024/06/12 21:23:23
use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
