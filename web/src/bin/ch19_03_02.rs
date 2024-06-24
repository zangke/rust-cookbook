//! Filename: ch19_03_02.rs
//! Description: 从文件名获取 MIME 类型
//! Date: 2024/06/16 20:49:07
use mime::Mime;

fn find_mimetype(filename: &String) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    return res;
}

fn main() {
    let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mime = find_mimetype(&file.to_owned());
        println!("MIME for {}: {}", file, mime);
    }
}
