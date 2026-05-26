# Day 5：集合和迭代器

## 先运行

```bash
cargo run --bin day05_collections
```

## 再读代码

重点阅读 `src/bin/day05_collections.rs`：

- `Vec<T>` 是可增长数组。
- `String` 是 UTF-8 字节序列。
- `HashMap<K, V>` 用 key 查 value。
- `iter()`、`filter()`、`collect()` 组成迭代器流水线。

## 底层原理

`Vec` 的元素连续放在堆上。容量不足时，它会申请更大的内存，把旧元素搬过去，再释放旧内存。所以频繁扩容可能有成本，已知数量时可以用 `Vec::with_capacity`。

`String` 存的是 UTF-8 字节，不支持直接按字符下标访问。因为一个中文字符通常占多个字节，直接用数字索引容易切到非法边界。

`HashMap` 的遍历顺序不稳定。示例里为了输出稳定，先收集成 `Vec` 再排序。

## 练习

- 用 `Vec::with_capacity(10)` 创建分数列表，观察容量。
- 把低于 90 的分数也筛选出来。
- 给 `HashMap` 增加一个商品，并修改已有商品数量。

## 常见坑

- `scores.iter()` 迭代的是引用，示例里用 `copied()` 变成整数值。
- `HashMap` 需要 `use std::collections::HashMap;`。
- 字符串长度 `len()` 返回字节数，不是用户看到的字符数。
