/// **变量**
pub fn let_variable() {
    // 一个变量默认为不可变的，并且会自动推导类型
    //在rust里，如果一个变量未使用：比如 a 和 b，如果没有使用在运行 build 的时候，rust 编译器会直接报错
    //如果不想被警告,可以在变量名前添加 _ 修饰，这里表示让 rust编译器忽略它

    // 自动推导
    let _a = 3;

    // 指定类型
    let _b: i64 = 332;

    // 可变的变量
    let mut c = 12;

    c += 24;

    println!("{}", c);
}

/// **返回数据**
/// 方法在计算完成后，可以添加 return xxx; 返回数据
///
/// 更简洁的写法: 可以直接省略 return 和 ; 直接写上 value
/// 这里就表示将 value 返回
pub fn return_value(number: i32) -> i32 {
    let a = 10_i32 * number;
    // 使用 return 的方式
    // return a + number;
    a + number
}

// struct Struct {
//     e: i32,
// }

/// **解构**
pub fn deconstruction() {
    let (a, mut b) = (true, false);
    println!("A:{} \t B:{}", a, b);
    b = true;
    println!("A:{} \t B:{}", a, b);

    // let (a, b, c, d, e);
    //
    // (a, b) = (1, 2);
    //
    // [c, .., d, _, _] = [1, 2, 3, 90, 9, 5];
    // Struct { e, .. } = Struct { e: 5 };
    //
    // println!("{} {} {} {}", a, b, c, d,);
    // println!("{}", return_value(e));

    let _n: i8 = i8::MAX;
    let _v: i8 = i8::MIN;
    print!("{}", _v + _n+1);
}

// pub fn constant() -> (i32, i32) {
//     const MAX_X: i32 = 12;
//     const MAX_Y: i32 = 44;
//
//     (MAX_X, MAX_Y)
// }
