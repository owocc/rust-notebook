#[no_mangle] // 禁用函数名混淆
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}