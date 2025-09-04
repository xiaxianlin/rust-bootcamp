use std::{
    env::{self, consts},
    path::Path,
};

fn switch_dir() -> std::io::Result<()> {
    let current = env::current_dir()?;
    println!("当前目录: {}", current.display());

    env::set_current_dir(Path::new("/tmp"))?;
    println!("新目录: {}", env::current_dir()?.display());

    Ok(())
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("所有的参数：{:?}", args);

    if args.len() > 1 {
        println!("第一个参数：{}", args[1]);
    } else {
        println!("无参数");
    }

    match env::var("PATH") {
        Ok(val) => println!("PATH: {}", val),
        Err(e) => println!("错误：{}", e),
    }

    unsafe {
        env::set_var("MY_VAR", "HELLO");
        println!("MY_VAR: {}", env::var("MY_VAR").unwrap());

        env::remove_var("MY_VAR");
        println!("移除后: {:?}", env::var("MY_VAR"));
    };

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    if switch_dir().is_err() {
        println!("目录切换失败");
    }

    if let Some(home) = home::home_dir() {
        println!("家目录: {}", home.display());
    } else {
        println!("无家目录");
    }

    println!("临时目录: {}", env::temp_dir().display());

    println!("OS: {}", consts::OS);
    println!("Arch: {}", consts::ARCH);
    println!("Family: {}", consts::FAMILY);
}
