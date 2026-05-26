fn main() {
    // 这个入口只做教程导航。每天的代码都放在 src/bin/dayXX_*.rs，
    // 这样可以用 `cargo run --bin 名称` 单独运行某一天的示例。
    println!("欢迎来到 7 天 Rust 新手教程");
    println!();
    println!("建议先运行：");
    println!("  cargo run --bin day01_basics");
    println!();
    println!("完整目录：");
    println!("  Day 1: cargo run --bin day01_basics");
    println!("  Day 2: cargo run --bin day02_ownership");
    println!("  Day 3: cargo run --bin day03_structs");
    println!("  Day 4: cargo run --bin day04_error_handling");
    println!("  Day 5: cargo run --bin day05_collections");
    println!("  Day 6: cargo run --bin day06_traits_tests");
    println!("  Day 7: cargo run --bin day07_todo_cli -- help");
    println!();
    println!("配套文档在 docs/day01.md 到 docs/day07.md。");
}
