//! Filename: ch09_01_05.rs
//! Description: 记录到 Unix 系统日志
//! Date: 2024/06/15 21:59:47
#[cfg(target_os = "linux")]
#[cfg(target_os = "linux")]
use syslog::{Error, Facility};

#[cfg(target_os = "linux")]
fn main() -> Result<(), Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;
    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("So far, only Linux systems are supported.");
}
