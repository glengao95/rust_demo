// 函数示例
// 定义函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 调用函数
fn main() {
    let result = add(3, 5);
    println!("3 + 5 的结果是: {}", result);
}

// 函数返回多个值
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}