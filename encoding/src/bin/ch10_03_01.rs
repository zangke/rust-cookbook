//! Filename: ch10_03_01.rs
//! Description: 对非结构化 JSON 序列化和反序列化
//! Date: 2024/06/23 08:34:30

use serde_json::json;
use serde_json::{Error, Value};

fn main() -> Result<(), Error> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}
