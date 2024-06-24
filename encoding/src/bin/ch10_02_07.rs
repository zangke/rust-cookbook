//! Filename: ch10_02_07.rs
//! Description: 用 Serde 将记录序列化为 CSV
//! Date: 2024/06/23 08:31:20

use error_chain::error_chain;
use serde::Serialize;
use std::io;

error_chain! {
   foreign_links {
       IOError(std::io::Error);
       CSVError(csv::Error);
   }
}

#[derive(Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record {
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())
}
