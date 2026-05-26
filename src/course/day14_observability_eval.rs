//! Day 14：可观测性、评估和回归门禁。

use crate::course_utils::print_banner;

pub fn run() {
    print_banner(14, "可观测性、评估和回归门禁");

    let cases = vec![
        EvalCase::new("高错误率必须进入报告", "error_rate=8%", true),
        EvalCase::new("普通日志不应误报事故", "info: health ok", false),
    ];

    let summary = run_eval(&cases);
    println!(
        "评估结果：passed={} failed={}",
        summary.passed, summary.failed
    );
}

#[derive(Debug)]
struct EvalCase {
    name: String,
    input: String,
    expect_incident: bool,
}

impl EvalCase {
    fn new(name: &str, input: &str, expect_incident: bool) -> Self {
        Self {
            name: name.to_string(),
            input: input.to_string(),
            expect_incident,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EvalSummary {
    passed: usize,
    failed: usize,
}

/// 简化版评估器：把规则输出和期望结果对比。
/// 真实项目可以把这里扩展为 JSONL cases、trace、耗时和失败样本落盘。
fn run_eval(cases: &[EvalCase]) -> EvalSummary {
    let mut passed = 0;
    let mut failed = 0;

    for case in cases {
        let predicted = detect_incident(&case.input);
        if predicted == case.expect_incident {
            passed += 1;
        } else {
            failed += 1;
            println!("失败用例：{}", case.name);
        }
    }

    EvalSummary { passed, failed }
}

fn detect_incident(input: &str) -> bool {
    input.contains("error_rate=8%") || input.contains("timeout")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_counts_passed_and_failed_cases() {
        let cases = vec![
            EvalCase::new("match", "timeout", true),
            EvalCase::new("mismatch", "health ok", true),
        ];

        assert_eq!(
            run_eval(&cases),
            EvalSummary {
                passed: 1,
                failed: 1
            }
        );
    }
}
