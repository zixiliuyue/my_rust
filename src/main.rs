fn main() {
    // 这个入口只做教程导航。每天的代码都放在 src/bin/dayXX_*.rs，
    // 这样可以用 `cargo run --bin 名称` 单独运行某一天的示例。
    println!("欢迎来到 15 天 Rust 工程化学习教程");
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
    println!("  Day 8: cargo run --bin day08_lifetimes");
    println!("  Day 9: cargo run --bin day09_modules_errors");
    println!("  Day 10: cargo run --bin day10_iterators_perf");
    println!("  Day 11: cargo run --bin day11_concurrency");
    println!("  Day 12: cargo run --bin day12_cli_config -- --service checkout-api --dry-run");
    println!("  Day 13: cargo run --bin day13_tool_policy");
    println!("  Day 14: cargo run --bin day14_observability_eval");
    println!("  Day 15: cargo run --bin day15_ops_agent");
    println!();
    println!("配套文档在 docs/day01.md 到 docs/day15.md。");
}
