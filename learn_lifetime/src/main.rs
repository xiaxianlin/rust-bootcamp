fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct Pair<'a, 'b> {
    left: &'a str,
    right: &'b str,
}

fn main() {
    println!("Hello, world!");

    let string1 = String::from("short");
    let result;
    {
        let string2 = String::from("longer");
        result = longest(&string1, &string2);
        println!("最长: {}", result); // 有效，在 string2 销毁前使用
    }

    let left = "left";
    {
        let right = "right";
        let pair = Pair {
            left: left,
            right: right,
        };
        println!("pair - left: {}, right: {}", pair.left, pair.right);
    }
    println!("left: {}", left);
}
