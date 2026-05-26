//! Day 1：Cargo、变量、基础类型和栈内存直觉。

use crate::course_utils::{explain_memory, print_banner};

pub fn run() {
    print_banner(1, "Cargo、变量和基础类型");

    // `let` 默认创建不可变绑定。不可变不是“值不能存在内存里”，
    // 而是编译器不允许通过这个名字再次修改它。
    let course_name = "Rust 15 天工程化教程";
    let day: u8 = 1;

    // 需要修改时显式写 `mut`。这让代码审查者一眼知道变量后面会变。
    let mut finished_lessons = 0;
    finished_lessons += 1;

    // Rust 有静态类型。多数时候编译器能推导类型；学习阶段可以主动标注。
    let score: i32 = 95;
    let temperature: f64 = 36.5;
    let is_beginner: bool = true;
    let first_letter: char = 'R';

    println!("课程：{course_name}");
    println!("今天是第 {day} 天，已完成小节：{finished_lessons}");
    println!("分数：{score}，温度：{temperature}，新手：{is_beginner}，首字母：{first_letter}");

    explain_memory("u8/i32/f64/bool/char", "栈", "大小在编译期固定，复制成本低");
    explain_memory(
        "&str 字面量",
        "程序只读数据区 + 栈上引用",
        "文本本体随程序一起编译，变量保存地址和长度",
    );
}
