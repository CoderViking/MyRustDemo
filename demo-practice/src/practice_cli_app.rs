use std::env;
use std::error::Error;
use std::fs;
use std::process;
use lib::Config;
use crate::lib;

pub fn run() {
    //
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // let query  = &args[1];
    // let filename = &args[2];

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // 使用标准错误输出宏，打印程序错误信息
        // 直接退出程序，不往后继续执行了
        process::exit(1);
    });

    println!("query: {}, filename: {}", config.query, config.filename);

    // let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    //
    // println!("The content of file: \n{}", content);
    if let Err(e) = lib::handle(config) {
        eprintln!("Application error: {}", e); // 使用标准错误输出宏，打印程序错误信息
        process::exit(1);
    }

}

// fn handle(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(&config.filename)?;
//     println!("The content of file: \n{}", contents);
//     Ok(())
// }
//
// struct Config {
//     query: String,
//     filename: String,
// }
// impl Config {
//     // 创建一个 Config 的构造函数
//     fn new (args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("参数数量不足");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//
//         Ok(Config{query, filename})
//     }
// }

