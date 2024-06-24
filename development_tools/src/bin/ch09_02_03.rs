//! Filename: ch09_02_03.rs
//! Description: 在日志信息中包含时间戳
//! Date: 2024/06/15 22:08:11
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}
