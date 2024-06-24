//! Filename: ch03_01.rs
//! Description: 使用 tar 包
//! Date: 2024/06/11 08:44:05

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar::Archive;

/// 压缩目录为 tar 包
fn park() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    //将 /var/log 目录内的内容递归添加到 backup/logs 路径下的归档文件中。
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

/// 解压 tar 包
fn unpark() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

// fn read_park() -> Result<()> {
//     let file = File::open("archive.tar.gz")?;
//     let mut archive = Archive::new(GzDecoder::new(file));
//     let prefix = "bundle/logs";

//     println!("Extracted the following files:");
//     archive
//         .entries()?
//         .filter_map(|e| e.ok())
//         .map(|mut entry| -> Result<PathBuf> {
//             let path = entry.path()?.strip_prefix(prefix)?.to_owned();
//             entry.unpack(&path)?;
//             Ok(path)
//         })
//         .filter_map(|e| e.ok())
//         .for_each(|x| println!("> {}", x.display()));

//     Ok(())
// }

fn main() -> Result<(), std::io::Error> {
    let _ = park();
    let _ = unpark();
    // let _ = read_park();
    Ok(())
}
