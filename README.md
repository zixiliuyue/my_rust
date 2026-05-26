# 15 天 Rust 工程化学习教程

这是一个能直接运行的 Rust 学习项目。课程参考 `/Users/hongsen.ren/code/github-code/llm-agent` 的 day-by-day 隔离式学习结构，并结合“平台工程 / SRE / 全栈工具链 / Agent 工程化 / 权限审批审计 / 可验证学习项目”的个人画像，设计成从入门到工程化实战的 15 天路线。

每一天都有独立文档和独立 binary，不 import 其他 day 的代码。最终项目是一个只使用 mock 数据的安全可审计 Ops Agent：它会演示证据收集、权限判断、命令风险分类、评估和审计，但不会访问真实生产系统、不会执行远程命令、不会写入敏感信息。

## 工程结构

这个仓库不是把代码都塞进 `main.rs` 的脚本式项目，而是按 Rust package/crate/module 的方式组织：

```text
hello-rust/
├── Cargo.toml                 # Cargo package 清单，声明 package、edition、默认 binary
├── src/
│   ├── lib.rs                 # library crate 入口，对外导出课程模块和实战模块
│   ├── main.rs                # 默认 binary，只打印课程索引
│   ├── bin/                   # 多个 binary crate，每天一个命令入口
│   ├── course/                # Day 1-14 的课程模块，承载核心教学逻辑和测试
│   ├── ops_agent/             # Day 15 final project，按 domain/evidence/policy/audit/workflow 分层
│   ├── todo.rs                # Day 7 Todo CLI 的核心业务模块
│   └── course_utils.rs        # 课程通用工具模块
├── docs/                      # 每天独立学习文档
└── examples/data/             # 本地运行数据目录
```

分层规则：

- `src/bin/dayXX_*.rs`：只做参数读取、退出码和调用库模块，不放核心业务。
- `src/course/dayXX_*.rs`：每天的主要学习代码和单元测试，彼此隔离。
- `src/ops_agent/`：最终实战项目的包式模块拆分，模拟真实服务里的 domain、policy、workflow 等边界。
- `src/lib.rs`：统一暴露库 API，让 binary、测试和后续扩展复用同一套逻辑。

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
# 结果判断：终端打印 15 天教程索引
# 风险：会在 target/ 下生成调试构建产物
cargo run
```

## 15 天路线

| 天数 | 主题 | 文档 | 运行命令 |
| --- | --- | --- | --- |
| Day 1 | Cargo、变量、基础类型、栈和执行流程 | `docs/day01.md` | `cargo run --bin day01_basics` |
| Day 2 | 所有权、移动、复制、借用、引用 | `docs/day02.md` | `cargo run --bin day02_ownership` |
| Day 3 | 函数、控制流、结构体、方法 | `docs/day03.md` | `cargo run --bin day03_structs` |
| Day 4 | enum、match、Option、Result | `docs/day04.md` | `cargo run --bin day04_error_handling` |
| Day 5 | Vec、String、HashMap、迭代器 | `docs/day05.md` | `cargo run --bin day05_collections` |
| Day 6 | 模块、泛型、trait、测试 | `docs/day06.md` | `cargo run --bin day06_traits_tests` |
| Day 7 | 命令行待办事项工具实战 | `docs/day07.md` | `cargo run --bin day07_todo_cli -- help` |
| Day 8 | 生命周期、切片、引用有效期 | `docs/day08.md` | `cargo run --bin day08_lifetimes` |
| Day 9 | 模块边界、自定义错误、Display | `docs/day09.md` | `cargo run --bin day09_modules_errors` |
| Day 10 | 迭代器、闭包、性能直觉 | `docs/day10.md` | `cargo run --bin day10_iterators_perf` |
| Day 11 | 线程、channel、并发边界 | `docs/day11.md` | `cargo run --bin day11_concurrency` |
| Day 12 | CLI 参数、配置默认值、失败显性化 | `docs/day12.md` | `cargo run --bin day12_cli_config -- --service checkout-api --dry-run` |
| Day 13 | 工具策略、权限审批、安全边界 | `docs/day13.md` | `cargo run --bin day13_tool_policy` |
| Day 14 | 可观测性、评估、回归门禁 | `docs/day14.md` | `cargo run --bin day14_observability_eval` |
| Day 15 | 最终实战：安全可审计 Ops Agent | `docs/day15.md` | `cargo run --bin day15_ops_agent` |

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

## 最终项目命令

```bash
# 用途：运行 Day 15 安全可审计 Ops Agent
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 参数含义：无参数，使用内置 mock 事故快照
# 结果判断：输出 service、status、risk、证据板、假设和审计记录
# 风险：只读 mock，不访问真实 Grafana/Prometheus/SSH/数据库/MCP，不执行命令
cargo run --bin day15_ops_agent
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
- `src/bin/day*.rs`：每天的可运行命令入口。
- `src/course/day*.rs`：Day 1-14 的课程模块和测试。
- `src/ops_agent/`：Day 15 最终实战模块包，包含 domain、evidence、policy、audit、workflow。
- `src/todo.rs`：Day 7 实战的核心逻辑和测试。
- `docs/day*.md`：每天的学习说明。
- `examples/data/`：Day 7 命令行工具的数据目录。

当前目录里原本存在 `src/main` 和 `src/rustcqrzvcO/symbols.o`，它们看起来是历史本地编译残留。本教程没有删除它们，只在 `.gitignore` 中补充忽略规则，避免后续误提交。

## 敏感信息约束

- 课程代码只使用标准库和本地 mock 数据。
- 不写入真实账号、密码、token、私钥、服务器地址或生产系统配置。
- Day 15 报告会对包含 `token=`、`password=`、`secret=` 的片段做最小脱敏演示。
- `.gitignore` 已忽略运行时数据、编译产物和本地对象文件，避免误提交本地生成物。
