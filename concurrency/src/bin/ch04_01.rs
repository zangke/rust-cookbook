//! Filename: ch04_01.rs
//! Description: 显式线程--生成短期线程
//! Date: 2024/06/12 08:46:22
//!
fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

/// 本实例将数组一分为二，并在不同的线程中并行计算。
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        println!("max_l: {} max_r: {}", max_l, max_r);

        Some(max_l.max(max_r))
    })
    .unwrap()
}
