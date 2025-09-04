use std::{
    fmt,
    fs::File,
    io::{self, BufRead, Error},
    num::ParseIntError,
};

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    Negative,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Parse(e) => write!(f, "解析错误：{}", e),
            MyError::Negative => write!(f, "负数无效"),
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> Self {
        MyError::Parse(value)
    }
}

fn parse_positive(s: &str) -> Result<i32, MyError> {
    let num: i32 = s.parse()?;
    if num < 0 {
        Err(MyError::Negative)
    } else {
        Ok(num)
    }
}

fn read_file_to_vec(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut data = vec![];
    for line in reader.lines() {
        data.push(line?);
    }
    Ok(data)
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("-1"));
    println!("{:?}", parse_positive("abc"));

    let data = read_file_to_vec("test.txt");
    if let Ok(data) = data {
        println!("读取内容");
        println!("{:?}", data);
    } else {
        println!("读取错误");
    }

    let res: Result<i32, Error> = Ok(5);
    let res1 = res.and_then(|x| Ok(x * 5));
    assert_eq!(res1.unwrap(), 25);

    let err: Result<i32, &str> = Err("错误");
    let err_mapped = err.map_err(|e| format!("新错误: {}", e));
    println!("{:?}", err_mapped);
}
