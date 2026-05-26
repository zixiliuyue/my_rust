//! 教学用 mock 输入。
//!
//! 这里不放真实主机、真实账号或真实 token，避免学习仓库携带敏感信息。

use super::domain::IncidentSnapshot;

/// 构造一份固定 mock 事故，便于 Day 15 示例稳定运行。
pub fn sample_incident() -> IncidentSnapshot {
    IncidentSnapshot {
        service: String::from("checkout-api"),
        error_rate: 0.082,
        latency_p95_ms: 1380,
        recent_release: Some(String::from("release-2026-05-26-1530")),
        suspicious_logs: vec![
            String::from("db timeout when loading payment profile"),
            String::from("retry budget exhausted for dependency billing-db"),
        ],
        requested_command: String::from("systemctl restart checkout-api"),
        operator: String::from("local-student"),
        has_production_permission: false,
    }
}
