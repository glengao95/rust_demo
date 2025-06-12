// 错误处理示例
// 可恢复错误: Result<T, E>
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("创建文件时出错: {:?}", e),
                }
            }
            other_error => panic!("打开文件时出错: {:?}", other_error),
        },
    };
}

// 不可恢复错误: panic!
fn main_panic() {
    panic!("发生不可恢复的错误!");
}

// 使用 ? 运算符简化错误处理
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}