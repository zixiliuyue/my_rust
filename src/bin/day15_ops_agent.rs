use hello_rust::course_utils::print_banner;
use hello_rust::ops_agent::{run_incident, sample_incident};

fn main() {
    print_banner(15, "最终实战：安全可审计 Ops Agent");

    let snapshot = sample_incident();
    let report = run_incident(&snapshot);

    println!("service={}", report.service);
    println!("status={}", report.status);
    println!("risk={}", report.command_decision.risk);
    println!("reason={}", report.command_decision.reason);

    println!("\n证据板：");
    for evidence in report.evidence {
        println!("- {}: {}", evidence.name, evidence.detail);
    }

    println!("\n假设：");
    for hypothesis in report.hypotheses {
        println!("- {hypothesis}");
    }

    println!("\n审计记录：");
    for item in report.audit_log {
        println!("- {item}");
    }
}
