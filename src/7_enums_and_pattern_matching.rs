// 枚举和模式匹配示例
// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

// 使用枚举
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    println!("路由类型: {:?}", ip_kind);
}

// 带数据的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 模式匹配
fn main_match() {
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => {
            println!("收到退出消息");
        }
        Message::Move { x, y } => {
            println!("移动到坐标 ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("写入内容: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("更改颜色为 ({}, {}, {})", r, g, b);
        }
    }
}