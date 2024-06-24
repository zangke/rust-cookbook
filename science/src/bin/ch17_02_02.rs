//! Filename: ch17_02_02.rs
//! Description: 验证正切（tan）等于正弦（sin）除以余弦（cos）
//! Date: 2024/06/19 08:36:45

fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
