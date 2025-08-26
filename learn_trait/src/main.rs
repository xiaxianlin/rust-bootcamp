use std::f32::consts::PI;

trait Area {
    fn get_area(&self) -> f32;
}

struct Circle {
    r: f32,
}

impl Area for Circle {
    fn get_area(&self) -> f32 {
        self.r * self.r * PI
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.width * self.height
    }
}

trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
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

fn sum(mut iter: impl Iterator<Item = u32>) -> u32 {
    let mut total: u32 = 0;
    while let Some(i) = iter.next() {
        total += i;
    }
    total
}

fn main() {
    let c1 = Circle { r: 20.0 };
    println!("Circle area: {}", c1.get_area());

    let r1 = Rectangle {
        width: 12.0,
        height: 12.0,
    };
    println!("Rectangle area: {}", r1.get_area());

    let counter = Counter { count: 0 };
    let total = sum(counter);
    println!("Total: {}", total);

    let areas: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { r: 30.0 }),
        Box::new(Rectangle {
            width: 20.0,
            height: 10.0,
        }),
    ];
    for a in areas {
        println!("{}", a.get_area());
    }
}
