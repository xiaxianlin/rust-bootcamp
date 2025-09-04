use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn run1() -> std::io::Result<()> {
    let mut child = Command::new("bc") // 交互计算器
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"2 + 3\n")?;
    drop(stdin); // 关闭输入

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("结果: {}", line?);
    }

    child.wait()?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
    run1().unwrap();
}
