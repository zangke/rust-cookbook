//! Filename: ch19_04_02.rs
//! Description: 为 REST 请求设置自定义消息标头和 URL 参数
//! Date: 2024/06/16 21:02:02
//! 
use error_chain::error_chain;
use serde::Deserialize;

use reqwest::header::USER_AGENT;
// use reqwest::header::{Authorization, Bearer, UserAgent};
use reqwest::Client;
use std::collections::HashMap;
use url::Url;

// header! { (XPoweredBy, "X-Powered-By") => [String] }

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let response = Client::new()
        .get(url)
        // .header(UserAgent::new("Rust-test"))
        .header(USER_AGENT, "Rust-test")
        // .header(Authorization(Bearer {
        //     token: "DEadBEEfc001cAFeEDEcafBAd".to_owned(),
        // }))
        .bearer_auth("DEadBEEfc001cAFeEDEcafBAd".to_owned())
        // .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .header("X-Powered-By", "Guybrush Threepwood".to_owned())
        .send()
        .await?;
    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );
    let out: HeadersEcho = response.json().await?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    // assert_eq!(
    //     response.url().as_str(),
    //     "http://httpbin.org/headers?lang=rust&browser=servo"
    // );

    println!("{:?}", out);
    Ok(())
}
