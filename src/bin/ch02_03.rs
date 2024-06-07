//! Filename: ch02_03.rs
//! Description: ANSI 终端
//! Date: 2024/06/07 08:48:19

use ansi_term::{Colour, Style};

/// 打印彩色文本到终端
fn print_color() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}

/// 终端中的粗体文本
fn print_bold() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}

/// 终端中的粗体和彩色文本
fn print_color_bold() {
    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
fn main() {
    print_color();
    print_bold();
    print_color_bold();
}
