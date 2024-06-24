//! Filename: ch17_01_07.rs
//! Description: （反）序列化矩阵
//! Date: 2024/06/18 08:55:24

extern crate nalgebra;
extern crate serde_json;

use nalgebra::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    // 序列化矩阵
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // 反序列化出矩阵
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    // 验证反序列化出的矩阵 `deserialized_matrix` 等同于原始矩阵 `matrix`
    assert!(deserialized_matrix == matrix);

    Ok(())
}
