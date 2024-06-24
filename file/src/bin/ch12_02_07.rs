//! Filename: ch12_02_07.rs
//! Description: 递归查找所有 png 文件
//! Date: 2024/06/23 06:44:11

use error_chain::error_chain;

use glob::glob;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

fn main() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
