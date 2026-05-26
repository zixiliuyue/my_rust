use hello_rust::course_utils::print_banner;

fn main() {
    print_banner(3, "函数、控制流、结构体和方法");

    let user = User::new(String::from("hongsen"), 28);
    user.say_hello();

    let rectangle = Rectangle {
        width: 30,
        height: 20,
    };

    println!("矩形面积：{}", rectangle.area());

    if rectangle.is_square() {
        println!("这是正方形");
    } else {
        println!("这是普通矩形");
    }

    for number in 1..=3 {
        println!("for 循环第 {number} 次，平方是 {}", square(number));
    }
}

/// 结构体把相关字段组织成一个类型。
/// 栈上的结构体字段会按编译器决定的布局存放；包含 `String` 时，字段本身仍是元数据。
struct User {
    name: String,
    age: u8,
}

impl User {
    /// `Self` 表示当前 `impl` 的类型，也就是 `User`。
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    /// `&self` 是 `self: &Self` 的简写，表示只读借用当前对象。
    fn say_hello(&self) {
        println!("你好，我是 {}，今年 {} 岁", self.name, self.age);
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn square(number: i32) -> i32 {
    // Rust 最后一行没有分号时会作为返回值。
    number * number
}
