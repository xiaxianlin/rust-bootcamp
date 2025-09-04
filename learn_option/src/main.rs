use std::env;

fn find_first_vowel(data: &str) -> Option<usize> {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for (index, b) in data.char_indices() {
        if vowels.iter().find(|i| *i == &b).is_some() {
            return Some(index);
        }
    }
    None
}

fn handle_avg(data: Option<Vec<i32>>) -> f32 {
    data.map(|v| {
        let sum = v.iter().sum::<i32>() as f32;
        let avg = sum / (v.len() as f32);
        avg
    })
    .unwrap_or(0.0f32)
}
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let vowel_index = args.iter().position(|x| x == "--vowel");
    let arg_index = args.iter().position(|x| x == "--arg");

    if vowel_index.is_some() {
        let data = "Hello";
        if let Some(i) = find_first_vowel(data) {
            println!("首个元音位置：{}", i + 1);
        } else {
            println!("未找到元音")
        }
    } else if arg_index.is_some() {
        let data = vec![1, 2, 3, 4, 5, 2];
        println!("平均值：{}", handle_avg(Some(data)));
    } else {
        println!("没有匹配到参数");
    }
}
