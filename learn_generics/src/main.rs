fn reverse<T: Clone>(data: &mut [T]) -> &[T] {
    data.reverse();
    data
}

fn reverse_munual<T: Clone>(data: &mut [T]) -> &[T] {
    let mut left = 0;
    let mut right = data.len() - 1;
    while left < right {
        data.swap(left, right);
        left += 1;
        right -= 1;
    }
    data
}

struct Pair<T: Copy> {
    left: T,
    right: T,
}

impl<T: Copy> Pair<T> {
    fn swap(&mut self) {
        let left = self.left;
        self.left = self.right;
        self.right = left;
    }
}

trait Printable {
    fn print(self);
}

struct Printer;
impl Printable for Printer {
    fn print(self) {
        println!("printer print!")
    }
}

struct Computer;
impl Printable for Computer {
    fn print(self) {
        println!("computer print!")
    }
}

fn print(item: impl Printable) {
    item.print();
}

fn main() {
    println!("Hello, world!");

    let mut data1 = [1, 2, 3];
    println!("data1: {:?}", reverse(&mut data1));

    let mut data2 = ["hello", "world"];
    println!("data2: {:?}", reverse_munual(&mut data2));

    let mut p = Pair {
        left: 10,
        right: 20,
    };
    println!("Pair - left: {}, right: {}", p.left, p.right);
    p.swap();
    println!("Pair swap - left: {}, right: {}", p.left, p.right);

    let printer = Printer;
    print(printer);
    let computer = Computer;
    print(computer);
}
