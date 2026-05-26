//! 15 天课程模块层。
//!
//! 每个 `dayXX_*` 文件是一个可单独学习、测试和复用的模块。
//! `src/bin/dayXX_*.rs` 只保留命令入口，真实教学逻辑放在这里，
//! 这就是 Rust 工程里常见的“库 crate 承载核心逻辑，binary crate 负责 I/O”的分层。

pub mod day01_basics;
pub mod day02_ownership;
pub mod day03_structs;
pub mod day04_error_handling;
pub mod day05_collections;
pub mod day06_traits_tests;
pub mod day08_lifetimes;
pub mod day09_modules_errors;
pub mod day10_iterators_perf;
pub mod day11_concurrency;
pub mod day12_cli_config;
pub mod day13_tool_policy;
pub mod day14_observability_eval;
