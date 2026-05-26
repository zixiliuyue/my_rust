# Day 10：迭代器、闭包和性能直觉

## 先运行

```bash
cargo run --bin day10_iterators_perf
```

## 再读代码

重点阅读 `src/bin/day10_iterators_perf.rs`：

- `filter` 只保留错误样本和慢请求。
- `map` 把结构体转换成人类可读摘要。
- `collect` 真正消费迭代器并生成 `Vec`。

## 底层原理

迭代器是惰性的。只写 `map` 或 `filter` 不会立刻执行，直到 `collect`、`sum`、`for` 这类消费动作发生。Rust 编译器通常能把迭代器链优化成接近手写循环的机器码。

## 练习

- 增加 `fold` 统计平均延迟。
- 增加服务名过滤条件。
- 写一个等价手写 `for` 循环，对比可读性。

## 常见坑

- `iter()` 产生引用，`into_iter()` 会消费集合。
- 闭包默认按需要借用、可变借用或移动捕获变量。
- 性能问题先用基准或 profiling 证明，不凭感觉重写。
