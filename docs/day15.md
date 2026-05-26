# Day 15：最终实战：安全可审计 Ops Agent

## 先运行

```bash
cargo run --bin day15_ops_agent
cargo test
```

## 再读代码

重点阅读：

- `src/ops_agent/mod.rs`：最终项目模块入口和对外 API。
- `src/ops_agent/domain.rs`：领域模型。
- `src/ops_agent/evidence.rs`：证据收集和脱敏。
- `src/ops_agent/policy.rs`：命令风险和权限边界。
- `src/ops_agent/audit.rs`：审计记录。
- `src/ops_agent/workflow.rs`：编排完整闭环。
- `src/bin/day15_ops_agent.rs`：CLI 展示入口。

## 项目目标

最终项目参考 `llm-agent` 的 SRE 排障、安全远程执行和 capstone 思路，但用 Rust 标准库实现一个本地教学版闭环：

- 输入 mock 事故快照。
- 收集错误率、延迟、发布和日志证据。
- 基于证据生成排障假设。
- 对请求命令做风险分类。
- 对敏感字段做最小脱敏。
- 输出可审计报告。

## 底层原理

这个项目刻意把模型可能参与的“总结/草稿”与必须确定的“权限/命令风险/审计/脱敏”分开。Rust 的类型系统负责固定数据边界，测试负责防止高危命令误放行和敏感信息泄漏。

工程结构上，`src/ops_agent/` 是一个包式模块：外部只通过 `hello_rust::ops_agent::run_incident` 使用它，内部可以继续拆分文件而不影响调用方。

## 练习

- 增加 `kubectl rollout undo` 的审批路径。
- 把 `OpsReport` 输出成 JSON 风格字符串。
- 增加一个“没有权限时不能进入执行状态”的测试。
- 增加 `Evidence` 的严重级别字段。

## 常见坑

- 最终项目默认只读 mock，不访问真实系统。
- 不要把真实 token、密码、服务器地址写进样例。
- 高危命令应该直接 blocked，不应该只靠“让模型谨慎一点”。
