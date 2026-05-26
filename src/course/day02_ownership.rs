//! Day 2：所有权、移动、复制和借用。

use crate::course_utils::{explain_memory, print_banner};

pub fn run() {
    print_banner(2, "所有权、移动和借用");

    // `String` 的文本内容在堆上，变量本身在栈上保存指针、长度、容量。
    let owner = String::from("hello ownership");
    explain_memory(
        "String 变量",
        "栈 + 堆",
        "栈上保存元数据，堆上保存可增长文本",
    );

    // 这里发生“移动”：`owner` 的所有权转移给 `new_owner`。
    // 移动后不能再使用 `owner`，否则两个变量会尝试释放同一块堆内存。
    let new_owner = owner;
    println!("新的所有者：{new_owner}");

    // `clone` 会复制堆上的文本内容，成本比移动高，但两个变量可以独立存在。
    let original = String::from("需要保留原值");
    let copied = original.clone();
    println!("original={original}, copied={copied}");

    // 借用不会拿走所有权。`&String` 是不可变引用，只能读，不能改。
    print_length(&original);
    println!("借用结束后，original 仍然可用：{original}");

    // 可变借用允许修改，但同一时间只能有一个可变引用。
    let mut note = String::from("Rust");
    append_word(&mut note, " 很严格，也很可靠");
    println!("修改后的 note：{note}");

    // 整数实现了 Copy，赋值时直接复制位，不会移动所有权。
    let a = 10;
    let b = a;
    println!("Copy 类型赋值后两个值都能用：a={a}, b={b}");
}

fn print_length(text: &String) {
    println!("'{text}' 的字节长度是 {}", text.len());
}

fn append_word(text: &mut String, word: &str) {
    text.push_str(word);
}
