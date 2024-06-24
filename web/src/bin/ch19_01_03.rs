//! Filename: ch19_01_03.rs
//! Description: 从 MediaWiki 标记页面提取所有唯一性链接
//! Date: 2024/06/16 20:27:40

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;

fn extract_links(content: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
        )
        .unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    links
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )
    .await?
    .text()
    .await?;

    println!("{:#?}", extract_links(content.as_str()));

    Ok(())
}
