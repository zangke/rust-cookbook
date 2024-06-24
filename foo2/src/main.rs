//! Filename: main.rs
//! Description: 编译 C 语言库时自定义设置
//! Date: 2024/06/16 08:44:16

extern "C" {
    fn print_app_info();
}

fn main() {
    unsafe {
        print_app_info();
    }
}
