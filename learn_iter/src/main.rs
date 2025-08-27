struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct Fib {
    prev: u32,
    curr: u32,
    index: u32,
}

impl Fib {
    fn new() -> Self {
        Fib {
            prev: 0,
            curr: 0,
            index: 0,
        }
    }
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let result;
        if self.index == 0 {
            result = Some(0);
        } else if self.index == 1 {
            self.curr = 1;
            result = Some(1);
        } else {
            let curr = self.curr;
            self.curr = self.curr + self.prev;
            self.prev = curr;
            result = Some(self.curr);
        }
        self.index += 1;
        result
    }
}

fn main() {
    println!("Hello, world!");

    let sum: u32 = Counter::new().sum();
    println!("总和: {}", sum);

    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let result: Vec<_> = data.iter().filter(|i| *i % 2 != 0).map(|i| i * 2).collect();
    println!("result: {:?}", result);

    let fib = Fib::new();
    let reuslt: Vec<_> = fib.take(10).collect();
    println!("Fib: {:?}", reuslt);

    let nums = vec![1, 2, 3];
    let strs = vec!["a", "b", "c"];

    for (i, &item) in nums.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    for (i, &item) in strs.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    for (&n, &s) in nums.iter().zip(strs.iter()) {
        println!("{} - {}", n, s);
    }
}
