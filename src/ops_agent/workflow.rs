//! Ops Agent 工作流编排。
//!
//! workflow 层只负责编排，不把证据、策略、审计逻辑混在一个函数里。

use super::audit::build_audit_log;
use super::domain::{IncidentSnapshot, OpsReport, RiskLevel};
use super::evidence::collect_evidence;
use super::hypothesis::build_hypotheses;
use super::policy::classify_command;

/// 完整运行一次教学版排障闭环。
///
/// 这里的关键原则是：模型可以参与总结，但权限、命令风险、证据格式和状态机必须由代码约束。
pub fn run_incident(snapshot: &IncidentSnapshot) -> OpsReport {
    let evidence = collect_evidence(snapshot);
    let hypotheses = build_hypotheses(snapshot, &evidence);
    let command_decision = classify_command(
        &snapshot.requested_command,
        snapshot.has_production_permission,
    );
    let audit_log = build_audit_log(snapshot, &command_decision);

    let status = match command_decision.risk {
        RiskLevel::Blocked => "blocked",
        RiskLevel::NeedsApproval => "ready-for-human-review",
        RiskLevel::SafeReadOnly => "dry-run-ready",
    }
    .to_string();

    OpsReport {
        service: snapshot.service.clone(),
        status,
        evidence,
        hypotheses,
        command_decision,
        audit_log,
    }
}
