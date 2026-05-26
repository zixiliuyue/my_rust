# Rust 模块和包工程实践

本仓库使用一个 Cargo package，同时包含一个 library crate 和多个 binary crate。

## package / crate / module

- `Cargo.toml` 定义 package：项目名、版本、edition、默认运行入口。
- `src/lib.rs` 定义 library crate：可复用的课程模块、Todo 业务模块、Ops Agent 模块都从这里导出。
- `src/main.rs` 和 `src/bin/*.rs` 定义 binary crate：负责命令入口，不承载核心业务。
- `mod.rs` 或同名 `.rs` 文件定义 module：用于拆分领域模型、策略、工作流和测试边界。

## 为什么 binary 要薄

如果把所有代码放进 `main()`：

- 单元测试只能绕命令行输出，很难测业务规则。
- 复用困难，后续 Web、CLI、测试会复制逻辑。
- 参数解析、业务规则、文件读写、权限策略混在一起，review 成本高。

本仓库采用：

```text
src/bin/day15_ops_agent.rs  ->  src/ops_agent/workflow.rs
                              ->  src/ops_agent/evidence.rs
                              ->  src/ops_agent/policy.rs
                              ->  src/ops_agent/audit.rs
                              ->  src/ops_agent/domain.rs
```

入口只负责展示结果，业务规则在库模块中测试。

## 可见性规则

- 默认私有：函数、结构体、字段不写 `pub` 时只能在当前模块或子模块内访问。
- 精确公开：只把 binary 或其他模块确实需要的 API 标成 `pub`。
- 通过 `mod.rs` 收口：外部使用 `hello_rust::ops_agent::run_incident`，不需要知道内部文件怎么拆。

## 本仓库的工程边界

- Day 1-14：放在 `src/course/`，每个 day 一个模块，学习主题隔离。
- Day 7：CLI 入口在 `src/bin/day07_todo_cli.rs`，核心业务在 `src/todo.rs`。
- Day 15：最终项目放在 `src/ops_agent/`，按真实工程职责拆层。

## 验证命令

```bash
# 用途：验证所有模块和 binary 都能编译，并运行单元测试
# 执行目录：/Users/hongsen.ren/code/github-code/rust/hello-rust
# 结果判断：无 FAIL，退出码为 0
# 风险：只生成本地构建缓存，不访问网络、不执行远程命令
cargo test
```
