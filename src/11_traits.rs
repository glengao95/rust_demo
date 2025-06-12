// 11_traits.rs
// Trait
//类似java接口
use std::fmt::Display;
use std::fmt::Debug;
// 定义Trait
pub trait Summary {
    fn summarize(&self) -> String;//&self类似java的this
}

// 实现Trait
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 默认实现
pub trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(阅读更多...)")
    }
}

// Trait作为参数
fn notify(item: &impl Summary) {
    println!("新闻快讯! {}", item.summarize());
}

// Trait Bound语法 必须实现了Summary
fn notify_with_summary<T: Summary>(item: &T) {
    println!("新闻快讯! {}", item.summarize());
}

// 多个Trait Bound  必须实现了Summary和Display
fn notify_multiple(item: &(impl Summary + Display)) {
    println!("新闻快讯! {}", item.summarize());
}

// 使用where子句
fn some_function<T, U>(t: &T, u: &U) -> i32
//T必须实现Display和Clone, U必须实现Clone和Debug
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 函数体
    0
}

// 返回实现Trait的类型  返回了Summart接口实现
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，你知道他们怎么说的..."),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("大新闻"),
        location: String::from("北京"),
        author: String::from("张三"),
        content: String::from("今天发生了一件大事..."),
    };

    println!("新闻摘要: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("李四"),
        content: String::from("今天天气真好"),
        reply: false,
        retweet: false,
    };

    println!("推文摘要: {}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}