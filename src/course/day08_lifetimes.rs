//! Day 8：生命周期、切片和引用有效期。

use crate::course_utils::print_banner;

pub fn run() {
    print_banner(8, "生命周期、切片和引用有效期");

    let incident = String::from("checkout-api error_rate high");
    let keyword = first_word(&incident);
    println!("第一段关键词：{keyword}");

    let left = String::from("billing-db timeout");
    let right = String::from("cache ok");
    println!("更长的描述：{}", longer(left.as_str(), right.as_str()));

    let note = Note {
        title: "生命周期不是延长引用",
        body: "它是告诉编译器多个引用之间的有效期关系",
    };
    println!("{}：{}", note.title, note.body);
}

/// 返回输入字符串的第一个单词切片。
///
/// 返回值 `&str` 借用自参数 `text`，所以它不能比 `text` 活得更久。
fn first_word(text: &str) -> &str {
    for (index, ch) in text.char_indices() {
        if ch == ' ' {
            return &text[..index];
        }
    }

    text
}

/// 显式生命周期 `'a` 表示：返回值引用来自 left 或 right，并且不能超过两者较短的有效期。
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

/// 结构体里保存引用时，必须声明引用字段的生命周期。
struct Note<'a> {
    title: &'a str,
    body: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_returns_slice() {
        assert_eq!(first_word("rust ownership"), "rust");
        assert_eq!(first_word("rust"), "rust");
    }
}
