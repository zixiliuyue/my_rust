# Day 13：工具策略、权限和安全边界

## 先运行

```bash
cargo run --bin day13_tool_policy
```

## 再读代码

重点阅读 `src/bin/day13_tool_policy.rs`：

- `PolicyDecision` 把工具调用结果分为 dry-run、需要审批、直接阻断。
- `decide` 用确定性规则分类命令风险。
- 高危命令不会进入审批队列，而是直接 blocked。

## 底层原理

参考 `llm-agent` 的工具策略思路：模型可以写审批摘要，但不能决定是否真的执行危险命令。路由、权限、过滤和执行边界必须由确定性代码控制。

## 练习

- 增加 `kubectl delete` 风险判断。
- 为数据库写操作增加 `NeedsApproval`。
- 把命令、操作者和决策组成审计结构体。

## 常见坑

- denylist 只能做入门演示，生产系统还要做 allowlist、参数解析和上下文校验。
- dry-run 不等于安全，仍然要记录操作者、目标和参数。
- 权限判断不能只放在前端或提示词里。
