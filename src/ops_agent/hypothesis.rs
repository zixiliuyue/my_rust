//! 基于证据生成排障假设。
//!
//! 教学版先用确定性规则，避免把路由、权限或数据转换交给模型自由发挥。

use super::domain::{Evidence, IncidentSnapshot};

/// 根据证据生成假设。这里故意不用模型，先把确定性规则写清楚。
pub fn build_hypotheses(snapshot: &IncidentSnapshot, evidence: &[Evidence]) -> Vec<String> {
    let mut hypotheses = Vec::new();

    if snapshot.error_rate > 0.05 {
        hypotheses.push(String::from("错误率超过 5%，优先检查依赖超时和近期发布"));
    }

    if snapshot.latency_p95_ms > 1000 {
        hypotheses.push(String::from("P95 延迟超过 1s，可能存在下游依赖退化"));
    }

    if evidence.iter().any(|item| item.detail.contains("timeout")) {
        hypotheses.push(String::from(
            "日志出现 timeout，先验证数据库或外部服务可用性",
        ));
    }

    if snapshot.recent_release.is_some() {
        hypotheses.push(String::from("存在近期发布，需要准备回滚候选和变更 diff"));
    }

    if hypotheses.is_empty() {
        hypotheses.push(String::from("暂无强证据，继续扩大只读采样范围"));
    }

    hypotheses
}
