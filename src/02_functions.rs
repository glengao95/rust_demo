// 02_functions.rs
// 函数

fn main() {
    // 简单函数
    print_hello();

    // 带参数的函数
    print_sum(5, 10);

    // 带返回值的函数
    let result = add(3, 4);
    println!("3 + 4 = {}", result);

    // 函数表达式
    let square = |x: i32| -> i32 { x * x };
    let square = |x| x * x;//也不以可不指定类型
    println!("5的平方是: {}", square(5));
}

// 无参数无返回值的函数
fn print_hello() {
    println!("Hello!");
}

// 带参数无返回值的函数
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// 带参数和返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}