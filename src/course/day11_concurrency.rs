//! Day 11：线程、channel 和并发边界。

use crate::course_utils::print_banner;
use std::sync::mpsc;
use std::thread;

pub fn run() {
    print_banner(11, "线程、消息传递和并发边界");

    let jobs = vec!["check logs", "check metrics", "check release"];
    let results = run_workers(jobs);

    for result in results {
        println!("{result}");
    }
}

fn run_workers(jobs: Vec<&'static str>) -> Vec<String> {
    let (sender, receiver) = mpsc::channel();

    for job in jobs {
        let sender = sender.clone();
        thread::spawn(move || {
            // move 把 job 所有权交给线程，避免线程引用主线程栈上的临时数据。
            let message = format!("{job}: ok");
            sender.send(message).expect("接收端应该还存在");
        });
    }

    drop(sender);

    let mut results: Vec<String> = receiver.iter().collect();
    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workers_return_all_results() {
        let results = run_workers(vec!["b", "a"]);

        assert_eq!(results, vec!["a: ok", "b: ok"]);
    }
}
