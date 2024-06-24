//! Filename: ch16_01_05.rs
//! Description: 持续处理子进程的输出
//! Date: 2024/06/17 08:59:35

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}
