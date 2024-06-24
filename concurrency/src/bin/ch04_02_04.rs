//! Filename: ch04_02_04.rs
//! Description: 对 vector 并行排序
//! Date: 2024/06/12 21:28:37

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn main() {
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
    });
    vec.par_sort_unstable();
}
