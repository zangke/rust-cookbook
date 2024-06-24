//! Filename: ch18_02_02.rs
//! Description: 自定义结构体并实现 FromStr trait
//! Date: 2024/06/16 09:27:34

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // 解析格式为 '#rRgGbB..' 的颜色十六进制代码
    // 将其转换为 'RGB' 实例
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32)
        // 将给定的字符串切片转换为 u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(RGB { r, g, b })
    }
}

fn main() {
    let code: &str = &r"#fa7268";
    match RGB::from_str(code) {
        Ok(rgb) => {
            println!(
                r"The RGB color code is: R: {} G: {} B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            println!("{} is not a valid color hex code!", code);
        }
    }

    // 测试 from_str 函数执行是否符合预期
    assert_eq!(
        RGB::from_str(&r"#fa7268").unwrap(),
        RGB {
            r: 250,
            g: 114,
            b: 104
        }
    );
}
