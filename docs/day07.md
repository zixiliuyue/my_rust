# Day 7：命令行待办事项工具实战

## 先运行

```bash
cargo run --bin day07_todo_cli -- help
cargo run --bin day07_todo_cli -- add "学习所有权"
cargo run --bin day07_todo_cli -- list
cargo run --bin day07_todo_cli -- done 1
cargo run --bin day07_todo_cli -- list
```

## 再读代码

重点阅读：

- `src/bin/day07_todo_cli.rs`：命令行参数解析和用户输出。
- `src/todo.rs`：待办事项数据结构、文件读写、业务逻辑和单元测试。
- `examples/data/`：运行时数据目录。

## 底层原理

命令行参数来自操作系统。Rust 用 `std::env::args()` 读取参数，第 0 个参数通常是程序路径，所以示例中用 `skip(1)` 跳过。

文件读写使用 `std::fs`。`read_to_string` 把文件读成 `String`，`write` 把字符串写回文件。示例采用一行一条记录的文本格式：`id<TAB>done<TAB>title`，这样可以直接观察序列化和反序列化过程。

业务逻辑放在 `src/todo.rs`，命令行入口只负责把用户输入转换成函数调用。这种分层让核心逻辑能用 `cargo test` 自动验证。

## 练习

- 增加 `delete 1` 命令，删除指定 id 的待办事项。
- 增加 `clear-done` 命令，清理已完成事项。
- 把列表输出改成先显示未完成，再显示已完成。
- 给新增命令补单元测试。

## 常见坑

- `add` 后面的标题如果有空格，需要用引号包起来。
- `done` 后面必须是数字 id。
- 数据文件 `examples/data/todos.txt` 是运行时生成的，已被 `.gitignore` 忽略。
