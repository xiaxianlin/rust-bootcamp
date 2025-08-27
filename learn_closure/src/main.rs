use std::thread;

fn create_counter(mut init: i32, step: i32) -> impl FnMut() -> i32 {
    move || {
        init += step;
        init
    }
}

fn main() {
    println!("Hello, world!");

    let data = "Hello, this is a thread";
    let handle = thread::spawn(move || println!("{}", data));

    let mut counter = create_counter(2, 1);
    println!("step1: {}", counter());
    println!("step2: {}", counter());

    handle.join().unwrap();

    let data = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<_> = data.iter().filter(|i| *i % 2 == 0).collect();
    println!("result: {:?}", result);
}
