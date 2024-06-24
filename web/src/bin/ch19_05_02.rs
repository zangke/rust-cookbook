//! Filename: ch19_05_02.rs
//! Description: 检查 API 资源是否存在
//! Date: 2024/06/16 22:05:01

use reqwest::ClientBuilder;
use reqwest::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
