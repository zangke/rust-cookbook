//! Filename: ch09_03_03.rs
//! Description: 检查给定版本是否为预发布版本
//! Date: 2024/06/15 22:16:14
use semver::{SemVerError, Version};

fn main() -> Result<(), SemVerError> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert!(version_1.is_prerelease());
    assert!(!version_2.is_prerelease());

    Ok(())
}
