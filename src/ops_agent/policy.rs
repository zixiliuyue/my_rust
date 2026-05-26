//! 工具策略和命令风险分类。
//!
//! 命令执行是高风险边界，必须由确定性代码控制，不能交给提示词兜底。

use super::domain::{CommandDecision, RiskLevel};

/// 对命令做风险分类。
pub fn classify_command(command: &str, has_production_permission: bool) -> CommandDecision {
    let normalized = command.trim().to_ascii_lowercase();

    if normalized.contains("rm -rf")
        || normalized.contains("mkfs")
        || normalized.contains("drop table")
        || normalized.contains("delete from")
        || normalized.contains("shutdown")
    {
        return CommandDecision {
            risk: RiskLevel::Blocked,
            reason: String::from("命中破坏性命令规则，不能进入审批队列"),
            dry_run: false,
        };
    }

    if normalized.starts_with("systemctl")
        || normalized.starts_with("supervisorctl")
        || normalized.starts_with("kubectl delete")
        || normalized.starts_with("mysql")
    {
        return CommandDecision {
            risk: RiskLevel::NeedsApproval,
            reason: if has_production_permission {
                String::from("涉及生产变更，需要人工审批和回滚方案")
            } else {
                String::from("操作者没有生产权限，只能生成审批摘要")
            },
            dry_run: true,
        };
    }

    CommandDecision {
        risk: RiskLevel::SafeReadOnly,
        reason: String::from("只读命令，可进入 dry-run 或本地演示"),
        dry_run: true,
    }
}
