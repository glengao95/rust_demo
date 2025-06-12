// 结构体示例
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建结构体实例
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("用户邮箱: {}", user1.email);
}

// 结构体方法
impl User {
    fn login(&mut self) {
        self.sign_in_count += 1;
        self.active = true;
    }
}