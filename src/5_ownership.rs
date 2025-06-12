// 所有权示例
// 变量作用域
fn main() {
    { // s 在这里无效, 它尚未声明
        let s = "hello"; // s 从这里开始有效
        println!("{}", s);
    } // 此作用域已结束, s 不再有效
}

// 移动语义
fn main_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 这行代码会报错，因为 s1 的所有权已经移动到 s2
    println!("{}", s2);
}

// 引用
fn main_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' 的长度是 {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}