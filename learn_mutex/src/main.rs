use std::panic;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

#[allow(unused)]
fn run1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap()); // 10
}

#[allow(unused)]
fn poison() {
    let mutex = Mutex::new(0);
    let _ = panic::catch_unwind(|| {
        let _guard = mutex.lock().unwrap();
        panic!("毒");
    });

    let guard = mutex.lock();
    if guard.is_err() {
        let poisoned = guard.unwrap_err();
        println!("毒: {}", mutex.is_poisoned()); // true
        let inner = poisoned.into_inner();
        println!("恢复: {}", *inner); // 0，忽略毒
    }
}

#[allow(dead_code)]
struct Data {
    a: i32,
    b: String,
}

#[allow(unused)]
fn run2() {
    let mutex = Mutex::new(Data {
        a: 1,
        b: "hello".to_string(),
    });
    let mut guard = mutex.lock().unwrap();
    {
        let a_mut = &mut guard.a;
        *a_mut += 10;
    }
    // let  a_mut = MutexGuard::map(guard, |d| &mut d.a);
    // *a_mut += 10;
    // drop(a_mut);
    // guard.b 仍可访问
    println!("b = {}", guard.b);
}

#[allow(unused)]
fn run3() {
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..10000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("计: {}", *counter.lock().unwrap());
    println!("线程时间: {:?}", start.elapsed()); // 分析争用
}

#[allow(unused)]
fn run3_1() {
    let start = Instant::now();
    let mut counter = 0;
    for _ in 0..1000000 {
        counter += 1;
    }
    println!("计: {}", counter);
    println!("直接时间: {:?}", start.elapsed());
}

#[allow(unused)]
fn run4() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("启动");
}

#[allow(unused)]
fn run5() {
    let state = Arc::new(Mutex::new("初始".to_string()));
    let state_clone = Arc::clone(&state);

    let handle = thread::spawn(move || {
        let mut guard = state_clone.lock().unwrap();
        *guard = "更新".to_string();
    });

    handle.join().unwrap();
    println!("状态: {}", *state.lock().unwrap());
}

fn main() {
    println!("Hello, world!");
    run3_1();
    run3();
}
