// 流程控制示例
fn main() {
    // if 语句
    let number = 5;
    if number < 10 {
        println!("数字小于10");
    } else {
        println!("数字大于等于10");
    }

    // loop 循环
    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        println!("计数器: {}", counter);
        counter += 1;
    }

    // while 循环
    let mut i = 0;
    while i < 3 {
        println!("while 计数器: {}", i);
        i += 1;
    }

    // for 循环
    for element in 1..4 {
        println!("for 循环元素: {}", element);
    }
}