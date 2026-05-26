use hello_rust::course_utils::print_banner;
use hello_rust::todo::{add_todo, list_todos, mark_done};
use std::env;
use std::path::PathBuf;

fn main() {
    print_banner(7, "实战：命令行待办事项工具");

    // `env::args()` 读取命令行参数。第 0 个参数是程序路径，所以先 skip(1)。
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| String::from("help"));
    let data_path = PathBuf::from("examples/data/todos.txt");

    let result = match command.as_str() {
        "add" => {
            let title = args.collect::<Vec<_>>().join(" ");
            if title.trim().is_empty() {
                Err(String::from(
                    "用法：cargo run --bin day07_todo_cli -- add \"学习所有权\"",
                ))
            } else {
                add_todo(&data_path, &title)
                    .map(|item| println!("已添加 #{}：{}", item.id, item.title))
                    .map_err(|error| format!("添加失败：{error}"))
            }
        }
        "list" => list_todos(&data_path)
            .map(print_items)
            .map_err(|error| format!("读取失败：{error}")),
        "done" => match args.next().and_then(|text| text.parse::<usize>().ok()) {
            Some(id) => mark_done(&data_path, id)
                .map(|updated| match updated {
                    Some(item) => println!("已完成 #{}：{}", item.id, item.title),
                    None => println!("没有找到 id={id} 的待办事项"),
                })
                .map_err(|error| format!("更新失败：{error}")),
            None => Err(String::from(
                "用法：cargo run --bin day07_todo_cli -- done 1",
            )),
        },
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        other => Err(format!("未知命令：{other}\n运行 help 查看用法")),
    };

    if let Err(message) = result {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn print_items(items: Vec<hello_rust::todo::TodoItem>) {
    if items.is_empty() {
        println!("暂无待办事项。先运行 add 命令添加一条。");
        return;
    }

    for item in items {
        let status = if item.done { "x" } else { " " };
        println!("[{status}] #{} {}", item.id, item.title);
    }
}

fn print_help() {
    println!("用法：");
    println!("  cargo run --bin day07_todo_cli -- add \"学习所有权\"");
    println!("  cargo run --bin day07_todo_cli -- list");
    println!("  cargo run --bin day07_todo_cli -- done 1");
    println!();
    println!("数据文件：examples/data/todos.txt");
}
