//! Ops Agent 领域模型。
//!
//! 领域模型只描述数据，不负责读取文件、访问网络或执行命令。

use std::fmt;

/// 一次事故或排障任务的输入快照。
///
/// 真实系统里这些字段可能来自 Prometheus、日志、发布系统和权限中心。
/// 教学项目里把它们写成普通结构体，方便先理解数据流和边界。
#[derive(Debug, Clone)]
pub struct IncidentSnapshot {
    pub service: String,
    pub error_rate: f64,
    pub latency_p95_ms: u32,
    pub recent_release: Option<String>,
    pub suspicious_logs: Vec<String>,
    pub requested_command: String,
    pub operator: String,
    pub has_production_permission: bool,
}

/// Agent 产出的证据。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub name: String,
    pub detail: String,
}

/// 命令风险等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    SafeReadOnly,
    NeedsApproval,
    Blocked,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::SafeReadOnly => write!(f, "safe-read-only"),
            RiskLevel::NeedsApproval => write!(f, "needs-approval"),
            RiskLevel::Blocked => write!(f, "blocked"),
        }
    }
}

/// 命令风险分类结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandDecision {
    pub risk: RiskLevel,
    pub reason: String,
    pub dry_run: bool,
}

/// 最终报告，适合作为 CLI 输出或写入审计日志。
#[derive(Debug, Clone)]
pub struct OpsReport {
    pub service: String,
    pub status: String,
    pub evidence: Vec<Evidence>,
    pub hypotheses: Vec<String>,
    pub command_decision: CommandDecision,
    pub audit_log: Vec<String>,
}
