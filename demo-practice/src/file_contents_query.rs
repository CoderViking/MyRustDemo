use std::error::Error;
use std::{env, fs, process};
use std::str::FromStr;

/// 启动说明：
/// 正常情况下启动时，使用 cargo run to readme.md 命令， 第一个参数是 query,第二个参数是文件名
/// 使用控制台环境变量启动的命令： CASE_INSENSITIVE=1 cargo run to readme.md ， 第一句是指明环境变量，用于控制启动程序的逻辑判断
/// 使用箭头命令，将程序的打印输出到指定的文件中：CASE_INSENSITIVE=1 cargo run to readme.md > output.md ， 其中的 > 之后指定的就是输出打印文件名
pub fn start() {
    let args: Vec<String> = env::args().collect();
    println!("程序启动参数：{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // 使用标准错误输出宏，打印程序错误信息
        // 直接退出程序，不往后继续执行了
        process::exit(1);
    });

    println!("query: {}, filename: {}", config.query, config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e); // 使用标准错误输出宏，打印程序错误信息
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    // println!("The content of file: \n{}", contents);
    Ok(())
}

struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    // 创建一个 Config 的构造函数
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("参数数量不足，请使用' cargo run query filename true|false '命令行启动程序");
        }
        let query = args[1].clone();
        let filename = args[2].clone();


        let case_sensitive = bool::from_str(&args[3].clone()).unwrap_or_else(|_| false); // 将字符串转换为布尔值, 存在没有参数时，下标越界的问题
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 从环境变量中获取该参数的值，如果环境变量中出现这个key 那么结果就为 true ，否则为 false
        Ok(Config{query, filename, case_sensitive})
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    // 遍历 contents
    let mut results = Vec::new();
    for line in  contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    // 遍历 contents
    let mut results = Vec::new();
    let query = query.to_lowercase(); // 将 query 转换为小写，此处会返回一个全新的有所有权的变量
    for line in  contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}

