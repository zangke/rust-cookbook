//! Filename: main.rs
//! Description: 检查逻辑 cpu 内核的数量
//! Date: 2024/06/20 08:38:49

fn main() {
    println!("Hello, hardware!");
    println!("Number of logical cores is {}", num_cpus::get());
}
