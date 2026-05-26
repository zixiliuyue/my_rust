//! 课程示例共用的小工具。
//!
//! 这里故意保持简单，方便新手先理解模块、函数和可见性。

/// 打印每天示例的标题。
///
/// `pub` 表示这个函数可以被其他模块调用；没有 `pub` 的函数默认只在当前模块可见。
pub fn print_banner(day: u8, title: &str) {
    println!("========== Day {day}: {title} ==========");
}

/// 用文字解释一个值通常会放在栈还是堆。
///
/// 注意：这是教学辅助，不是编译器接口。真实布局还会受到优化、平台 ABI 等影响。
pub fn explain_memory(name: &str, location: &str, reason: &str) {
    println!("{name}: 通常在 {location}，原因：{reason}");
}
