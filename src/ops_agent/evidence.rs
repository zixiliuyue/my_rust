//! 证据收集和脱敏。
//!
//! 真实项目里证据收集应当是只读查询，不应该在采集阶段修复系统。

use super::domain::{Evidence, IncidentSnapshot};

/// 收集证据。真实项目里这一步应该是只读查询，不应该顺手修复。
pub fn collect_evidence(snapshot: &IncidentSnapshot) -> Vec<Evidence> {
    let mut evidence = Vec::new();

    evidence.push(Evidence {
        name: String::from("error_rate"),
        detail: format!("{:.2}%", snapshot.error_rate * 100.0),
    });

    evidence.push(Evidence {
        name: String::from("latency_p95"),
        detail: format!("{}ms", snapshot.latency_p95_ms),
    });

    if let Some(release) = &snapshot.recent_release {
        evidence.push(Evidence {
            name: String::from("recent_release"),
            detail: release.clone(),
        });
    }

    for log in &snapshot.suspicious_logs {
        evidence.push(Evidence {
            name: String::from("log"),
            detail: redact_sensitive(log),
        });
    }

    evidence
}

/// 最小敏感信息脱敏。
///
/// 这里只演示原则：任何 token、password、secret 都不能原样进入报告。
pub fn redact_sensitive(input: &str) -> String {
    let mut output = Vec::new();

    for part in input.split_whitespace() {
        let lower = part.to_ascii_lowercase();
        let looks_sensitive = ["token", "password", "secret"]
            .iter()
            .any(|key| lower.contains(key) && lower.contains('='));

        if looks_sensitive {
            output.push("[REDACTED]");
        } else {
            output.push(part);
        }
    }

    output.join(" ")
}
