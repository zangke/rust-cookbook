//! Filename: ch04_01_05.rs
//! Description: 对所有 iso 文件的 SHA256 值并发求和
//! Date: 2024/06/12 09:01:53

use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};

// Verify the iso extension
fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
        _ => false,
    }
}

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

fn main() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
