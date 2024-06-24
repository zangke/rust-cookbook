//! Filename: ch10_02_06.rs
//! Description: 将记录序列化为 CSV
//! Date: 2024/06/23 08:29:55

use error_chain::error_chain;

use std::io;

error_chain! {
    foreign_links {
        CSVError(csv::Error);
        IOError(std::io::Error);
    }
}

fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}
