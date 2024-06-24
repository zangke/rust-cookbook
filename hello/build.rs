//! Filename: build.rs
//! Description: 构建脚本
//! Date: 2024/06/16 08:11:09

fn main() {
    cc::Build::new().file("src/hello.c").compile("hello"); // 输出 `libhello.a`
}
