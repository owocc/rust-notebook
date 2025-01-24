mod basic_syntax;
use basic_syntax::{deconstruction, let_variable, return_value};

/// **入口方法**
///
/// 这里是程序的入口方法
///
/// 当程序运行时会先调用这个方法
fn main() {
    let_variable();
    println!("{}", return_value(32_i32));
    deconstruction();
}
