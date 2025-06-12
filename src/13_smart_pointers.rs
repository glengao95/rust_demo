// Rust智能指针示例

// 1. Box<T> - 在堆上分配值
fn box_example() {
    let b = Box::new(5); // 在堆上分配一个整数
    println!("b = {}", b);
}

// 2. Rc<T> - 引用计数智能指针(单线程)
use std::rc::Rc;

fn rc_example() {
    let rc1 = Rc::new(String::from("共享数据"));
    {
        let rc2 = Rc::clone(&rc1);
        println!("rc2: {}", rc2);
    }
    println!("rc1: {}", rc1);
    println!("引用计数: {}", Rc::strong_count(&rc1));
}

// 3. Arc<T> - 原子引用计数智能指针(多线程)
use std::sync::Arc;
use std::thread;

fn arc_example() {
    let arc1 = Arc::new(42);
    let mut handles = vec![];
    
    for _ in 0..5 {
        let arc_clone = Arc::clone(&arc1);
        handles.push(thread::spawn(move || {
            println!("线程中的值: {}", arc_clone);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    println!("主线程中的值: {}", arc1);
}

// 4. RefCell<T> - 运行时检查的不可变/可变借用
use std::cell::RefCell;

fn refcell_example() {
    let cell = RefCell::new(5);
    {
        let mut m = cell.borrow_mut(); // 获取可变引用
        *m += 1; // 修改内部值
    }
    println!("cell = {}", cell.borrow()); // 获取不可变引用并打印
}

// 5. Cell<T> - 内部可变性(无需借用检查)
use std::cell::Cell;

fn cell_example() {
    let cell = Cell::new(5);
    cell.set(10); // 直接修改值
    println!("cell value: {}", cell.get()); // 获取值
}

// 6. Mutex<T> - 线程安全的内部可变性
use std::sync::Mutex;

fn mutex_example() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // 获取锁
        *num = 6; // 修改值
    } // 锁自动释放
    println!("m = {:?}", m);
}

// 7. Mutex + Arc 多线程示例
fn mutex_arc_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    println!("最终计数: {}", *counter.lock().unwrap());
}

fn main() {
    box_example();
    rc_example();
    arc_example();
    refcell_example();
}