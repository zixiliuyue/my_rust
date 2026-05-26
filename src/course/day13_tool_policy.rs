//! Day 13：工具策略、权限和安全边界。

use crate::course_utils::print_banner;

pub fn run() {
    print_banner(13, "工具策略、权限和安全边界");

    for command in ["tail -n 20 app.log", "systemctl restart api", "rm -rf /"] {
        let decision = decide(command);
        println!("{command} => {decision:?}");
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PolicyDecision {
    AllowDryRun,
    NeedsApproval,
    Blocked,
}

/// 策略判断必须是确定性代码，不能让模型自由决定是否执行危险命令。
fn decide(command: &str) -> PolicyDecision {
    let command = command.to_ascii_lowercase();

    if command.contains("rm -rf") || command.contains("drop table") {
        PolicyDecision::Blocked
    } else if command.starts_with("systemctl") || command.starts_with("mysql") {
        PolicyDecision::NeedsApproval
    } else {
        PolicyDecision::AllowDryRun
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dangerous_command_is_blocked() {
        assert_eq!(decide("rm -rf /"), PolicyDecision::Blocked);
    }
}
