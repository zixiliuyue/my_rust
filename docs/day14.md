# Day 14：可观测性、评估和回归门禁

## 先运行

```bash
cargo run --bin day14_observability_eval
```

## 再读代码

重点阅读 `src/bin/day14_observability_eval.rs`：

- `EvalCase` 表达输入和期望结果。
- `run_eval` 统计通过和失败。
- `detect_incident` 是可替换的检测规则。

## 底层原理

工程化 Agent 不能只看一次 demo 输出。需要固定 eval case、trace、失败样本和回归门禁，才能知道改动是否真的变好。

## 练习

- 把 eval case 改成从文本文件读取。
- 输出失败用例名称和输入。
- 增加一个“不能泄漏 secret”的评估用例。

## 常见坑

- 评估数据要覆盖正常路径、错误路径和边界条件。
- 指标名要稳定，方便 CI 或脚本读取。
- 不要用“看起来对”替代可重复验证。
