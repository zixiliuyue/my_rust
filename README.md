# 7 天 Rust 新手教程

这是一个能直接运行的 Rust 入门项目。每天都有一份文档和一个独立可执行示例，适合从零开始学习语法、内存模型、错误处理、测试和小型命令行实战。

## 环境检查

```bash
# 用途：确认当前机器是否已经安装 Rust 编译器和 Cargo
# 执行目录：任意目录
# 结果判断：能看到 rustc/cargo 版本号
# 风险：只读命令，没有写入风险
rustc --version
cargo --version
```

当前项目使用 `edition = "2024"`。如果你的 Rust 版本过旧，请先升级 Rust 工具链。

## 快速开始

```bash
# 用途：运行教程总入口，查看每天的命令
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 结果判断：终端打印 7 天教程索引
# 风险：会在 target/ 下生成调试构建产物
cargo run
```

## 7 天路线

| 天数 | 主题 | 文档 | 运行命令 |
| --- | --- | --- | --- |
| Day 1 | Cargo、变量、基础类型、栈和执行流程 | `docs/day01.md` | `cargo run --bin day01_basics` |
| Day 2 | 所有权、移动、复制、借用、引用 | `docs/day02.md` | `cargo run --bin day02_ownership` |
| Day 3 | 函数、控制流、结构体、方法 | `docs/day03.md` | `cargo run --bin day03_structs` |
| Day 4 | enum、match、Option、Result | `docs/day04.md` | `cargo run --bin day04_error_handling` |
| Day 5 | Vec、String、HashMap、迭代器 | `docs/day05.md` | `cargo run --bin day05_collections` |
| Day 6 | 模块、泛型、trait、测试 | `docs/day06.md` | `cargo run --bin day06_traits_tests` |
| Day 7 | 命令行待办事项工具实战 | `docs/day07.md` | `cargo run --bin day07_todo_cli -- help` |

## 实战案例命令

```bash
# 用途：添加一条待办事项
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 参数含义：add 表示新增，后面的字符串是待办标题
# 输入示例：学习所有权
# 结果判断：输出“已添加 #编号：标题”
# 风险：会写入 examples/data/todos.txt
cargo run --bin day07_todo_cli -- add "学习所有权"

# 用途：列出全部待办事项
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 参数含义：list 表示查看列表
# 结果判断：输出 [ ] 或 [x] 开头的待办列表
# 风险：只读取 examples/data/todos.txt
cargo run --bin day07_todo_cli -- list

# 用途：把 id=1 的待办事项标记为完成
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 参数含义：done 表示完成，1 是待办事项 id
# 结果判断：输出“已完成 #1：标题”
# 风险：会更新 examples/data/todos.txt
cargo run --bin day07_todo_cli -- done 1
```

## 验证命令

```bash
# 用途：检查代码格式
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 结果判断：退出码为 0，没有格式化差异
# 风险：只检查，不会改文件
cargo fmt --check

# 用途：运行全部单元测试
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 结果判断：无 FAIL，退出码为 0
# 风险：会生成测试构建缓存；Day 7 测试会在系统临时目录创建并清理测试文件
cargo test
```

## 常见报错

- `command not found: cargo`：没有安装 Rust，先安装 rustup 或系统包里的 Rust。
- `no bin target named ...`：命令里的 binary 名称写错，按上方表格复制。
- `Permission denied`：检查项目目录或 `examples/data/` 是否有写权限。
- `invalid digit found in string`：Day 7 的 `done` 命令需要数字 id，例如 `done 1`。

## 文件说明

- `src/main.rs`：教程总入口。
- `src/bin/day*.rs`：每天的可运行示例。
- `src/todo.rs`：Day 7 实战的核心逻辑和测试。
- `docs/day*.md`：每天的学习说明。
- `examples/data/`：Day 7 命令行工具的数据目录。

当前目录里原本存在 `src/main` 和 `src/rustcqrzvcO/symbols.o`，它们看起来是历史本地编译残留。本教程没有删除它们，只在 `.gitignore` 中补充忽略规则，避免后续误提交。
