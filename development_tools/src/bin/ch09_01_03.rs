//! Filename: ch09_01_03.rs
//! Description: 记录信息时，用标准输出 stdout 替换标准错误 stderr
//! Date: 2024/06/15 21:56:54

use env_logger::{Builder, Target};

fn main() {
    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");
}
