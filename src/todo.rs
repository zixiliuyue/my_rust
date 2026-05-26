//! Day 7 实战：待办事项核心逻辑。
//!
//! 这个模块不直接读取命令行参数，只负责数据结构、文件读写和业务规则。
//! 命令行入口放在 `src/bin/day07_todo_cli.rs`，两者分开后更容易测试。

use std::fs;
use std::io;
use std::path::Path;

/// 一条待办事项。
///
/// `#[derive(...)]` 会让编译器自动生成常用能力：
/// - `Debug`：方便 `println!("{:?}", item)` 调试。
/// - `Clone`：必要时可以显式复制结构体。
/// - `PartialEq, Eq`：测试里可以直接比较两个值是否相等。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

impl TodoItem {
    /// 构造函数不是 Rust 语法要求，只是社区常见约定。
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }
}

/// 添加一条待办事项并写回文件。
pub fn add_todo(path: &Path, title: &str) -> io::Result<TodoItem> {
    let mut items = load_todos(path)?;
    let next_id = items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
    let item = TodoItem::new(next_id, title.trim().to_string());
    items.push(item.clone());
    save_todos(path, &items)?;
    Ok(item)
}

/// 读取全部待办事项。
pub fn list_todos(path: &Path) -> io::Result<Vec<TodoItem>> {
    load_todos(path)
}

/// 把指定 id 的事项标记为完成。
///
/// 返回 `Ok(Some(item))` 表示找到了并更新成功；
/// 返回 `Ok(None)` 表示文件可读写，但没有这个 id。
pub fn mark_done(path: &Path, id: usize) -> io::Result<Option<TodoItem>> {
    let mut items = load_todos(path)?;
    let mut updated = None;

    for item in &mut items {
        if item.id == id {
            item.done = true;
            updated = Some(item.clone());
            break;
        }
    }

    if updated.is_some() {
        save_todos(path, &items)?;
    }

    Ok(updated)
}

/// 从文件加载待办事项。
///
/// 文件不存在时返回空列表，这是命令行小工具常见的友好默认行为。
pub fn load_todos(path: &Path) -> io::Result<Vec<TodoItem>> {
    match fs::read_to_string(path) {
        Ok(content) => parse_todos(&content),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}

/// 把待办事项保存到文件。
///
/// `create_dir_all` 会递归创建父目录，目录已存在时也会成功。
pub fn save_todos(path: &Path, items: &[TodoItem]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    for item in items {
        content.push_str(&format!(
            "{}\t{}\t{}\n",
            item.id,
            if item.done { "1" } else { "0" },
            escape_title(&item.title)
        ));
    }

    fs::write(path, content)
}

/// 解析纯文本存储格式。
///
/// 每行格式是：`id<TAB>done<TAB>title`。这里不用 JSON，是为了让新手能直接看到
/// 标准库字符串切分、错误处理和数据转换的基本写法。
fn parse_todos(content: &str) -> io::Result<Vec<TodoItem>> {
    let mut items = Vec::new();

    for (line_index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.splitn(3, '\t');
        let id_text = parts.next().unwrap_or_default();
        let done_text = parts.next().unwrap_or_default();
        let title_text = parts.next().unwrap_or_default();

        let id = id_text.parse::<usize>().map_err(|_| {
            invalid_data(format!("第 {} 行 id 不是数字：{id_text}", line_index + 1))
        })?;

        let done = match done_text {
            "0" => false,
            "1" => true,
            _ => {
                return Err(invalid_data(format!(
                    "第 {} 行完成状态必须是 0 或 1：{done_text}",
                    line_index + 1
                )));
            }
        };

        items.push(TodoItem {
            id,
            title: unescape_title(title_text),
            done,
        });
    }

    Ok(items)
}

fn invalid_data(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

/// 为了保持一行一条记录，把标题里的反斜杠、换行和 Tab 做最小转义。
fn escape_title(title: &str) -> String {
    title
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}

fn unescape_title(title: &str) -> String {
    let mut result = String::new();
    let mut chars = title.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            result.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => result.push('\n'),
            Some('t') => result.push('\t'),
            Some('\\') => result.push('\\'),
            Some(other) => {
                result.push('\\');
                result.push(other);
            }
            None => result.push('\\'),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_todo_path(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间不应该早于 UNIX_EPOCH")
            .as_nanos();
        std::env::temp_dir().join(format!("hello_rust_{name}_{nanos}.txt"))
    }

    #[test]
    fn add_and_list_todos() {
        let path = temp_todo_path("add_and_list");

        let first = add_todo(&path, "学习所有权").expect("添加第一条待办应该成功");
        let second = add_todo(&path, "练习 match").expect("添加第二条待办应该成功");
        let items = list_todos(&path).expect("读取待办列表应该成功");

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "学习所有权");
        assert!(!items[0].done);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn mark_done_updates_existing_item() {
        let path = temp_todo_path("mark_done");
        add_todo(&path, "写 Rust 测试").expect("添加待办应该成功");

        let updated = mark_done(&path, 1)
            .expect("更新待办应该成功")
            .expect("id=1 应该存在");
        let items = list_todos(&path).expect("读取待办列表应该成功");

        assert_eq!(updated.id, 1);
        assert!(items[0].done);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn title_escape_roundtrip() {
        let path = temp_todo_path("escape");
        let title = "第一行\n第二行\t带 Tab 和 \\";

        add_todo(&path, title).expect("添加包含特殊字符的标题应该成功");
        let items = list_todos(&path).expect("读取待办列表应该成功");

        assert_eq!(items[0].title, title);

        let _ = fs::remove_file(path);
    }
}
