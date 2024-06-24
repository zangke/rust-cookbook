//! Filename: ch08_01_01.rs
//! Description: 测量运行时间
//! Date: 2024/06/14 08:51:39
use std::thread;
use std::time::{Duration, Instant};

fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
