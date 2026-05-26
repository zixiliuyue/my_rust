//! Day 5：Vec、String、HashMap 和迭代器。

use crate::course_utils::{explain_memory, print_banner};
use std::collections::HashMap;

pub fn run() {
    print_banner(5, "Vec、String、HashMap 和迭代器");

    // Vec 是可增长数组。元素连续存放在堆上，变量本身保存指针、长度、容量。
    let mut scores = Vec::new();
    scores.push(80);
    scores.push(95);
    scores.push(88);

    println!("scores={scores:?}");
    println!(
        "长度 len={}，容量 capacity={}",
        scores.len(),
        scores.capacity()
    );
    explain_memory("Vec<T>", "栈 + 堆", "栈上保存元数据，堆上连续保存元素");

    // String 是 UTF-8 字节序列，不能随便按“第几个字符”索引。
    let text = String::from("Rust 你好");
    println!("字符串字节长度：{}", text.len());
    println!("逐个 Unicode 标量值输出：");
    for ch in text.chars() {
        println!("  {ch}");
    }

    // HashMap 用哈希表按 key 查 value。遍历顺序不稳定，所以展示前先排序。
    let mut inventory = HashMap::new();
    inventory.insert("book", 3);
    inventory.insert("keyboard", 1);
    inventory.insert("coffee", 2);

    let mut entries: Vec<_> = inventory.iter().collect();
    entries.sort_by_key(|(name, _count)| *name);

    for (name, count) in entries {
        println!("{name}: {count}");
    }

    // 迭代器是惰性的：map/filter 只是描述流程，collect/sum/for 才真正消费。
    let high_scores: Vec<i32> = scores
        .iter()
        .copied()
        .filter(|score| *score >= 90)
        .collect();
    println!("高分列表：{high_scores:?}");
}
