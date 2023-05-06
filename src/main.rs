use std::{env,process};

use minigrep::Config;

fn main() {
    // std::env::args返回一个传递给程序的命令行参数的迭代器
    // 迭代器生成一系列的值，在迭代器上调用collect方法将其转为一个集合
    // 其中第一个元素为当二进制文件的名称，后续的元素为输入的参数
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            // eprintln!宏将错误信息写入标准错误
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


