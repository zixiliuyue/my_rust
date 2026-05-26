# Day 6：模块、泛型、trait 和测试

## 先运行

```bash
cargo run --bin day06_traits_tests
cargo test
```

## 再读代码

重点阅读：

- `src/lib.rs`：库 crate 入口。
- `src/course/mod.rs`：课程模块树，统一导出 Day 1-14。
- `src/course/day06_traits_tests.rs`：Day 6 的核心逻辑和测试。
- `src/course_utils.rs`：共享模块。
- `src/bin/day06_traits_tests.rs`：薄 binary 入口，只调用课程模块。

## 底层原理

Rust 项目可以同时有库 crate 和多个 binary crate。`src/lib.rs` 暴露库代码，`src/bin/*.rs` 是多个独立可执行入口。binary 可以通过包名转换后的 crate 名访问库代码，本项目包名 `hello-rust` 在代码里写作 `hello_rust`。

本仓库已经把 Day 1-14 的核心逻辑放进 `src/course/`，`src/bin/` 只保留入口。这样测试可以直接覆盖库模块，后续如果要接 Web、HTTP API 或更复杂 CLI，不需要复制业务代码。

泛型函数 `fn bigger<T: Ord + Copy>(...)` 在编译期根据实际类型生成代码，这通常叫静态分发。好处是运行时开销低，坏处是编译产物可能变大。

测试函数放在 `#[cfg(test)] mod tests` 中，只有测试构建时才会编译。

## 练习

- 给 `Summary` 增加一个 `short_summary()` 默认方法。
- 给新的结构体 `Video` 实现 `Summary`。
- 为 `bigger` 增加字符串切片的测试。

## 常见坑

- 模块没有 `pub` 时，外部不能访问。
- trait 只定义能力，不自动保存字段。
- 测试里可以访问父模块的私有项，因为测试模块是当前文件的子模块。
