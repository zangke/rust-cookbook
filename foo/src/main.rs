//! Filename: main.rs
//! Description: 编译并静态链接到绑定的 C++ 语言库
//! Date: 2024/06/16 08:39:20

extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", multiply(5, 7));
    }
}
