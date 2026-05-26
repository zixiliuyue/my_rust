# Day 4：enum、match、Option 和 Result

## 先运行

```bash
cargo run --bin day04_error_handling
```

## 再读代码

重点阅读 `src/bin/day04_error_handling.rs`：

- `enum TaskStatus` 表示有限状态集合。
- `match` 对每个状态做分支处理。
- `Option<T>` 表示可能有值，也可能没有值。
- `Result<T, E>` 表示可能成功，也可能失败。
- `?` 可以把错误提前返回。

## 底层原理

Rust 没有 `null`。可能缺失的值必须写成 `Option`，调用方必须处理 `Some` 和 `None`。这样空值风险从运行时异常提前到编译期检查。

`Result` 把错误作为普通值传递，不依赖异常跳转。`?` 不是吞掉错误，而是把 `Err` 原样向上返回，让调用链显式表达失败路径。

## 练习

- 给 `TaskStatus` 增加一个 `Blocked` 状态，并更新 `label()`。
- 修改 `parse_port`，拒绝小于 1024 的端口。
- 写一个 `safe_first_char(text: &str) -> Option<char>`。

## 常见坑

- `match` 必须覆盖所有分支，新增 enum 变体后相关代码可能需要一起改。
- `unwrap()` 学习阶段可以试，但真实代码里要谨慎使用。
- `?` 只能用在返回 `Result` 或兼容类型的函数中。
