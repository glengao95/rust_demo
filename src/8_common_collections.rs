// 常见集合及操作示例
use std::collections::{HashMap, VecDeque};

fn main() {
    // Vec 向量集合
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vec 中的元素: {:?}", v);

    // HashMap 哈希映射
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Blue 队分数: {:?}", scores.get("Blue"));

    // VecDeque 双端队列
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("双端队列元素: {:?}", deque);
}