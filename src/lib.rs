//! 教程中可复用的库代码。
//!
//! `src/bin/*.rs` 是每天的可执行示例，`src/lib.rs` 暴露可复用模块。
//! 这样做可以演示 Rust 项目里“库 crate + 多个 binary crate”的常见组织方式。

pub mod course;
pub mod course_utils;
pub mod ops_agent;
pub mod todo;
