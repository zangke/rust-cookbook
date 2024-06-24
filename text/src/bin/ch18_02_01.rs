//! Filename: ch18_02_01.rs
//! Description: 收集 Unicode 字符
//! Date: 2024/06/16 09:24:12

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "天下为公 孙中山\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "公");
}
