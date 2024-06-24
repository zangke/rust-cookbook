//! Filename: ch19_02_05.rs
//! Description: 从 URL 移除片段标识符和查询对
//! Date: 2024/06/16 20:45:12

use url::{ParseError, Position, Url};

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned); //cleaned: https://github.com/rust-lang/rust/issues
    Ok(())
}
