# Day 9：模块边界和自定义错误

## 先运行

```bash
cargo run --bin day09_modules_errors
```

## 再读代码

重点阅读 `src/bin/day09_modules_errors.rs`：

- `src/bin/day09_modules_errors.rs` 是薄入口，只调用课程模块。
- `src/course/day09_modules_errors.rs` 承载 `Config::parse`、`ConfigError` 和测试。
- `ConfigError` 用 enum 表达所有失败原因，`Display` 负责把错误变成人能读懂的中文信息。

## 底层原理

工程代码里不要把所有错误都压成字符串。枚举错误可以被测试、被模式匹配，也能在未来转换成 API 状态码或 CLI 退出码。

模块边界上，binary 不应该直接堆满解析逻辑。把解析逻辑放到 library crate 后，测试和未来其他入口可以复用同一套代码。

## 练习

- 增加 `timeout_ms` 配置项。
- 为未知 key 增加建议文案。
- 把 `ConfigError` 拆到独立模块，练习 `pub` 可见性。

## 常见坑

- `?` 传播错误前，错误类型必须兼容。
- `Display` 面向用户，`Debug` 面向开发调试。
- 默认值要明确写在解析逻辑里，不要藏在调用方。
