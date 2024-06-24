//! Filename: ch19_02_01.rs
//! Description: 解析 URL 字符串为 Url 类型
//! Date: 2024/06/16 20:32:18

use url::{ParseError, Url};

fn main() -> Result<(), ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}
