use std::{
    fs::File,
    io::{self, Read, Write},
};

#[allow(unused)]
fn input() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    println!("你输入: {}", input.trim());
    Ok(())
}

#[allow(unused)]
fn line_input() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut count = 0;

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                count += 1;
                println!("行 {}: {}", count, line);
            }
            Some(Err(e)) => return Err(e), // 传播错误
            None => break,                 // EOF（如 Ctrl+D）
        }
    }
    println!("总行数: {}", count);
    Ok(())
}

#[allow(unused)]
fn output() -> io::Result<()> {
    let mut stdout = io::stdout().lock(); // 锁定以高效写入
    let mut stderr = io::stderr();

    write!(&mut stdout, "正常输出: ")?;
    writeln!(&mut stdout, "值 {}", 42)?;
    stdout.flush()?; // 立即发送

    writeln!(&mut stderr, "错误消息")?;
    Ok(())
}

#[allow(unused)]
fn read() -> io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut buffer = [0u8; 1024]; // 固定缓冲
    let bytes_read = file.read(&mut buffer)?;
    println!("读取 {} 字节: {:?}", bytes_read, &buffer[..bytes_read]);
    Ok(())
}

#[allow(unused)]
fn write() -> io::Result<()> {
    let mut file = File::create("output.bin")?;
    let data = b"binary data";
    let bytes_written = file.write(data)?;
    assert_eq!(bytes_written, data.len()); // 检查完整写入
    file.flush()?;
    Ok(())
}

#[derive(Debug)]
struct ReverseReader<'a>(&'a [u8]);

impl<'a> Read for ReverseReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = std::cmp::min(buf.len(), self.0.len());
        for (i, &byte) in self.0[..n].iter().rev().enumerate() {
            buf[i] = byte;
        }
        self.0 = &self.0[n..];
        Ok(n)
    }
}

fn reverse_reader() -> io::Result<()> {
    let data = b"hello";
    let mut reader = ReverseReader(data);
    let mut buf = vec![0; 5];
    reader.read_exact(&mut buf)?;
    println!("反转读取: {:?}", String::from_utf8_lossy(&buf)); // "olleh"
    Ok(())
}

fn main() {
    println!("Hello, world!");
    reverse_reader().unwrap();
}
