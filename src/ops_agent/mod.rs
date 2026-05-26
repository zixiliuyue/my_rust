//! Day 15 final project：教学版 Ops Agent 包模块。
//!
//! 这个目录刻意按真实工程拆层：
//! - `domain`：领域模型和公共类型。
//! - `fixtures`：教学用 mock 输入。
//! - `evidence`：证据收集和脱敏。
//! - `hypothesis`：基于证据生成假设。
//! - `policy`：命令风险和权限边界。
//! - `audit`：审计记录。
//! - `workflow`：把各模块串成完整闭环。

mod audit;
mod domain;
mod evidence;
mod fixtures;
mod hypothesis;
mod policy;
mod workflow;

pub use domain::{CommandDecision, Evidence, IncidentSnapshot, OpsReport, RiskLevel};
pub use evidence::{collect_evidence, redact_sensitive};
pub use fixtures::sample_incident;
pub use hypothesis::build_hypotheses;
pub use policy::classify_command;
pub use workflow::run_incident;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructive_command_is_blocked() {
        let decision = classify_command("rm -rf /", true);

        assert_eq!(decision.risk, RiskLevel::Blocked);
        assert!(!decision.dry_run);
    }

    #[test]
    fn production_restart_requires_approval() {
        let decision = classify_command("systemctl restart checkout-api", false);

        assert_eq!(decision.risk, RiskLevel::NeedsApproval);
        assert!(decision.reason.contains("没有生产权限"));
    }

    #[test]
    fn report_redacts_sensitive_log_parts() {
        let mut snapshot = sample_incident();
        snapshot
            .suspicious_logs
            .push(String::from("request failed token=abc123"));

        let report = run_incident(&snapshot);
        let joined = report
            .evidence
            .iter()
            .map(|item| item.detail.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(joined.contains("[REDACTED]"));
        assert!(!joined.contains("abc123"));
    }
}
