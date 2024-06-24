//! Filename: ch12_01_01.rs
//! Description: 读取文件的字符串行
//! Date: 2024/06/23 06:10:10

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
