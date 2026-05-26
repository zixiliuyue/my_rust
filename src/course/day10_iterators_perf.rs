//! Day 10：迭代器、闭包和性能直觉。

use crate::course_utils::print_banner;

pub fn run() {
    print_banner(10, "迭代器、闭包和零成本抽象");

    let samples = vec![
        MetricSample::new("checkout-api", 120, false),
        MetricSample::new("checkout-api", 1500, true),
        MetricSample::new("billing-db", 900, false),
    ];

    let report = summarize_slow_errors(&samples, 1000);
    println!("慢错误样本：{report:?}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetricSample {
    service: String,
    latency_ms: u32,
    error: bool,
}

impl MetricSample {
    fn new(service: &str, latency_ms: u32, error: bool) -> Self {
        Self {
            service: service.to_string(),
            latency_ms,
            error,
        }
    }
}

/// 迭代器链通常会被编译器优化成接近手写循环的机器码。
/// 业务上要先写清楚“过滤什么、映射什么、收集什么”，再考虑微优化。
fn summarize_slow_errors(samples: &[MetricSample], threshold_ms: u32) -> Vec<String> {
    samples
        .iter()
        .filter(|sample| sample.error)
        .filter(|sample| sample.latency_ms >= threshold_ms)
        .map(|sample| format!("{}:{}ms", sample.service, sample.latency_ms))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_only_slow_errors() {
        let samples = vec![
            MetricSample::new("a", 10, true),
            MetricSample::new("b", 2000, true),
            MetricSample::new("c", 3000, false),
        ];

        assert_eq!(summarize_slow_errors(&samples, 1000), vec!["b:2000ms"]);
    }
}
