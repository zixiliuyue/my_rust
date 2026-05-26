use hello_rust::course_utils::print_banner;

fn main() {
    print_banner(6, "模块、泛型、trait 和测试");

    let article = Article {
        title: String::from("Rust 为什么重视所有权"),
        author: String::from("学习者"),
    };

    let number = 42;

    print_summary(&article);
    print_summary(&number);
    println!("较大的数是 {}", bigger(7, 11));
}

/// trait 定义“某个类型能做什么”。
/// 这里的 Summary 类似其他语言里的接口，但 Rust 会在编译期检查实现是否完整。
trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("《{}》by {}", self.title, self.author)
    }
}

impl Summary for i32 {
    fn summary(&self) -> String {
        format!("数字 {self}")
    }
}

/// `T: Summary` 是 trait bound，表示传进来的类型必须实现 Summary。
/// 这种泛型通常会静态分发：编译器为具体类型生成对应版本，运行时不需要查表。
fn print_summary<T: Summary>(value: &T) {
    println!("{}", value.summary());
}

fn bigger<T: Ord + Copy>(left: T, right: T) -> T {
    if left >= right { left } else { right }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_summary_contains_title_and_author() {
        let article = Article {
            title: String::from("测试标题"),
            author: String::from("测试作者"),
        };

        assert_eq!(article.summary(), "《测试标题》by 测试作者");
    }

    #[test]
    fn bigger_returns_greater_value() {
        assert_eq!(bigger(3, 9), 9);
        assert_eq!(bigger('z', 'a'), 'z');
    }
}
