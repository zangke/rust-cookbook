//! Filename: ch09_01_01.rs
//! Description: 记录调试信息到控制台
//! Date: 2024/06/15 21:46:17

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
