use std::fmt::{self, Write};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("Hello, world!");

    let p = Point { x: 3, y: 4 };
    println!("Display: {}", p);
    println!("Debug: {:?}", p);
    println!("美化 Debug: {:#?}", p);

    let name = "Rust";
    let version = 1.80;
    let s = format!("{} 版本: {:.2}", name, version);
    println!("{}", s);

    let mut writer = String::new();
    write!(&mut writer, "整数: {:05}", 42).unwrap();
    writeln!(&mut writer, "\n十六进制: {:x}", 255).unwrap();
    println!("{}", writer);
}
