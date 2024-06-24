//! Filename: ch09_02_04.rs
//! Description: 将信息记录到自定义位置
//! Date: 2024/06/15 22:09:38
use error_chain::error_chain;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        LogConfig(log4rs::config::Errors);
        SetLogger(log::SetLoggerError);
    }
}

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    Ok(())
}
