//! Day 4：enum、match、Option 和 Result。

use crate::course_utils::print_banner;

pub fn run() {
    print_banner(4, "enum、match、Option 和 Result");

    let statuses = [TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::Done];
    for status in statuses {
        println!("任务状态：{}", status.label());
    }

    match find_user_name(1) {
        Some(name) => println!("找到用户：{name}"),
        None => println!("没有找到用户"),
    }

    match parse_port("8080") {
        Ok(port) => println!("端口解析成功：{port}"),
        Err(message) => println!("端口解析失败：{message}"),
    }

    match divide(10.0, 0.0) {
        Ok(value) => println!("除法结果：{value}"),
        Err(message) => println!("除法失败：{message}"),
    }
}

/// `enum` 表示一个值只能是几个变体之一。
/// 编译器会要求 `match` 覆盖所有可能分支，减少遗漏状态的 bug。
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    fn label(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "未开始",
            TaskStatus::InProgress => "进行中",
            TaskStatus::Done => "已完成",
        }
    }
}

fn find_user_name(id: u32) -> Option<&'static str> {
    if id == 1 {
        Some("Alice")
    } else {
        // Rust 没有 null。可能不存在的值用 `Option` 显式表达。
        None
    }
}

fn parse_port(text: &str) -> Result<u16, String> {
    let port = text
        .parse::<u16>()
        .map_err(|_| format!("'{text}' 不是合法端口数字"))?;

    Ok(port)
}

fn divide(left: f64, right: f64) -> Result<f64, String> {
    if right == 0.0 {
        Err(String::from("除数不能是 0"))
    } else {
        Ok(left / right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_cover_all_statuses() {
        assert_eq!(TaskStatus::Todo.label(), "未开始");
        assert_eq!(TaskStatus::InProgress.label(), "进行中");
        assert_eq!(TaskStatus::Done.label(), "已完成");
    }
}
