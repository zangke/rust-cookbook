//! Filename: ch17_02_01.rs
//! Description: 计算三角形的边长
//! Date: 2024/06/19 08:33:25

fn main() {
    let angle: f64 = 2.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}
