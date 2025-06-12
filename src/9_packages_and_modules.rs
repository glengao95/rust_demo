// 包和模块示例
// 定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("已添加到等待列表");
        }
    }
}

// 使用模块
fn main() {
    front_of_house::hosting::add_to_waitlist();
}

// 使用外部包示例（需在 Cargo.toml 中添加依赖）
// 例如添加 rand = "0.8.5" 到 [dependencies]
// use rand::Rng;
// fn main_random() {
//     let num: u8 = rand::thread_rng().gen_range(1..101);
//     println!("随机数: {}", num);
// }