//! Filename: ch16_01_06.rs
//! Description: 读取环境变量
//! Date: 2024/06/17 08:59:39

use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // 从环境变量 `CONFIG` 读取配置路径 `config_path`。
    // 如果 `CONFIG` 未设置，采用默认配置路径。
    let config_path = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
