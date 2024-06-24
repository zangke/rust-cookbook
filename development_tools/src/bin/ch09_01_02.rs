//! Filename: ch09_01_02.rs
//! Description: 记录错误信息到控制台
//! Date: 2024/06/15 21:54:55

fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}
