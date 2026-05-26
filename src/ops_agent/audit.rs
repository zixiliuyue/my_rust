//! 审计记录生成。
//!
//! 审计日志要记录“谁、对什么服务、做了什么风险判断”，但不能泄漏敏感信息。

use super::domain::{CommandDecision, IncidentSnapshot};
use super::evidence::redact_sensitive;

pub fn build_audit_log(snapshot: &IncidentSnapshot, decision: &CommandDecision) -> Vec<String> {
    vec![
        format!("operator={}", redact_sensitive(&snapshot.operator)),
        format!("service={}", snapshot.service),
        format!("risk={}", decision.risk),
        format!("dry_run={}", decision.dry_run),
        format!("reason={}", decision.reason),
    ]
}
