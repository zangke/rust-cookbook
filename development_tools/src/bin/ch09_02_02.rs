//! Filename: ch09_02_02.rs
//! Description: 用自定义环境变量设置日志记录
//! Date: 2024/06/15 22:06:18

use env_logger::Builder;
use std::env;

fn main() {
    Builder::new()
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}
