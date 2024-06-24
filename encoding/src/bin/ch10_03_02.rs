//! Filename: ch10_03_02.rs
//! Description: 反序列化 TOML 配置文件
//! Date: 2024/06/23 08:35:40

use toml::{de::Error, Value};

fn main() -> Result<(), Error> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(
        package_info["package"]["name"].as_str(),
        Some("your_package")
    );

    Ok(())
}
