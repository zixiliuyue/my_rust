# Day 12：CLI 参数和配置默认值

## 先运行

```bash
cargo run --bin day12_cli_config -- --service checkout-api --dry-run
```

## 再读代码

重点阅读 `src/bin/day12_cli_config.rs`：

- `env::args().skip(1)` 读取用户参数。
- `CliConfig::from_args` 把字符串参数转换为结构化配置。
- 错误路径会打印用法并以非零状态退出。

## 底层原理

CLI 的第一层职责是把不可信字符串输入转换成明确类型。真实项目里，参数解析、环境变量、配置文件和默认值要有清晰优先级，并且失败要显性化。

## 练习

- 增加 `--timeout-ms 1000`。
- 增加 `--env test|prod`，prod 时强制要求 `--dry-run`。
- 为无效参数补测试。

## 常见坑

- `--` 后面的参数才会传给 binary。
- 不要在业务函数里反复读 `env::args()`，入口解析一次即可。
- 配置默认值要能在 README 里讲清楚。
