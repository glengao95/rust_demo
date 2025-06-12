// 09_error_handling.rs
// 错误处理

fn main() {
    // 不可恢复错误
    // panic!("这是一个不可恢复错误");

    // 可恢复错误
    let result = read_file("hello.txt");
    match result {
        Ok(content) => println!("文件内容: {}", content),
        Err(error) => println!("读取文件失败: {}", error),
    }

    // 使用unwrap和expect
    // let content = read_file("hello.txt").unwrap();//unwarp 确定有值时使用，如果没有会panic
    let content = read_file("hello.txt").expect("读取文件失败");//如果读取失败，程序会panic并显示"读取文件失败"的错误信息
    println!("文件内容: {}", content);

    // 传播错误 会捕获错误，将错误传播给调用者，不会panic
    match propagate_error() {
        Ok(()) => println!("操作成功"),
        Err(error) => println!("操作失败: {}", error),
    }

    // ?操作符
    // let content = read_file_with_question_mark("hello.txt")?;
    // println!("文件内容: {}", content);
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

fn propagate_error() -> Result<(), std::io::Error> {
    let content = read_file("hello1.txt")?;
    println!("文件内容: {}", content);
    Ok(())
}

fn read_file_with_question_mark(filename: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
}