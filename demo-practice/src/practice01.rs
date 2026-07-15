use std::fmt::{Debug, Display};
use std::result;

pub fn run() {
    let mut s = String::from("Hello");
    s.push_str(" Rust");
    s.push_str(" Language");
    println!("{}", s);

    let x = 50;
    let y = x;
    println!("{}, {}", x, y);
    let s1 = String::from("自定义字符串变量");
    let s2 = s1; // 将s1的值赋值给s2时，只是将s1变量存储在栈中的指针地址、长度和容量信息拷贝了一份给s2,
                        // 但并没有把指针地址对应的堆上的数据进行拷贝，所以两个指针现在指向同一块堆内存。
                        // 这里由于两个变量指向了同一块堆内存地址，所以当变量离开作用域时，会发生两次内存资源释放的操作，
                        // 正常情况下一块内存只允许申请一次+释放一次，否则就会产生严重的安全性bug，所以这里就和资源释放的逻辑冲突了。
                        // rust 设计者针对这个问题采取了解决方案，它使s1的值在赋值给s2之后，自动将 s1变量设置为失效，且不清理内存资源。
    // println!("s1 = {}, s2 = {}", s1, s2); // 所以s1赋值给s2后，再次在s1的作用域内使用变量s1时就会编译报错。
    // 上面的赋值方式在其他语言中可以被称之为 浅拷贝 ，也叫复制引用，但是在rust中，它就被称为移动，因为变量s1的有效性比移动到了s2变量上面，s1自动失效了
    // rust 当然也支持对于堆内数据的深拷贝，可以直接调用clone()方法来实现。
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3); // 由于s3是拷贝了s2的栈和堆的数据，所以s3是一个独立的值，和s2不再有关联

    takes_ownership(s3);
    // println!("s3 = {}", s3); // 这里会编译报错，因为s3在前一句调用函数时已经发生了所有权移动，因此s3变量在调用函数后已经是无效变量
    makes_copy(y);
    println!("y = {}", y);

    let s4 = give_ownership();
    println!("s4 = {}", s4);
    let s5 = String::from("主函数内的字符串变量");
    let s6 = takes_and_give_back(s5);
    println!("s5 = {}", s6);
    let s7 = String::from("这个字符串长度是多少?");
    let (s8, len) = calculate_length(s7);
    // println!("s7 = {}", s7); // 报错，因为s7被传递给函数之后，已失去所有权，无法再次被使用
    println!("s8 = {}, len = {}", s8, len);

    let s9 = String::from("通过引用传递的字符串");
    let len = calculate_length1(&s9); // 将s9的引用传递给函数，而不是值，所以此处的s9仍然持有数据的所有权
    println!("s9 = {}, s9.len() = {}", s9, len); //
    let mut s10 = String::from("可以被改变的字符串");
    changes(&mut s10);
    println!("s10 = {}", s10);

    let s11 = String::from("用于赋值给两个变量的字符串");

    // let

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Viking"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
    };
    println!("New article available! {}", news_article.summarize());
    println!("author: {}", news_article.summarize_default_impl());
    notify(&news_article);
    notify1(&news_article);
    notify2(&news_article);
    println!("notify: {}", notify3(&news_article));
    println!("{}", news_article.format_print());

    let tweet = Tweet {
        username: String::from("Viking"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("author: {}", tweet.summarize_default_impl());
    notify(&tweet);
    notify1(&tweet);
    // notify2(&tweet); // 如果没有为 Tweet数据结构实现 Debug trait，则会编译错误：没有为 `Tweet` 实现特征 `Debug` 

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // 调用的函数中显式使用'a 标注引用的生命周期，因此在函数中，会使用生命周期最短的那个引用作为返回值的生命周期，因此在这段代码外执行result的打印是，会出现生命周期不够长的 错误
    }
    // println!("longest string = {}", result); // 错误因第81行代码的函数调用，导致返回值的引用的生命周期不足，在此处无法使用result变量


}

// 获取所有权函数，在rust中，发起函数调用和赋值是一样的，都会使变量的所有权发生移动
fn takes_ownership(some_string: String) {
    println!("函数内部使用变量：{}", some_string);
}
// 栈内确定数据大小的值，会在赋值时直接复制一份栈内的值，所以不会发生所有权移动
fn makes_copy(some_integer: i32) {
    println!("函数内部使用变量：{}",some_integer);
}
// 返回所有权
fn give_ownership() -> String {
    let some_string = String::from("返回值的字符串");
    some_string
}

// 将传入的字符串返回给函数调用方
fn takes_and_give_back(some_string: String) -> String {
    some_string
}

// 使用元祖返回多个值
fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

// rust中方法不能重名，即使参数和返回值不一样也不可以
fn calculate_length1(s: &String) -> usize {
    s.len()
}
fn changes(s: &mut String) {
    s.push_str(", 在字符串中新增一段字符串");
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
// #[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// 定一个 trait 命名为 Summary 并在其中定义多个不同的方法
// trait 可以理解为接口类型
trait Summary {
    // 声明一个方法，这个方法必须在实现该trait的数据类型中自定义实现这个方法才可以使用
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // 声明一个有默认实现的方法，实现该trait的数据结构可以不必重新实现这个方法，也可以使用
    fn summarize_default_impl(&self) -> String {
        println!("called trait default  impl method");
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 为 NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// 为 Tweet 实现 Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize_default_impl(&self) -> String {
        println!("called trait custom  impl method");
        format!("(更多内容来自作者: {}...)", self.summarize_author())
    }
}

// 函数接受实现了 Summary trait的数据类型作为参数
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 使用 trait bound 语法声明实现了指定 trait的数据类型
fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 声明函数参数需要是满足同时实现多个trait的数据类型
fn notify2<T: Summary + Debug>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 声明函数的参数限定数据类型时，可以使用where从句来将多个trait的限定放到函数定义的后面，避免影响函数的观感
fn notify3<T>(item: &T) -> String where T: Summary + Debug {
    format!("Breaking news! {}", item.summarize())
}

trait FormatPrint {
    fn format_print(&self) -> String;
}
// 为满足条件的任何泛型类型实现 FormatPrint trait
impl<T: Summary + Debug> FormatPrint for T {
    fn format_print(&self) -> String {
        format!("{:?}", &self)
    }
}

// 使用生命周期标注符号 'a，显式标注引用的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn a_unit_test() {
    println!("This is a unit test function");
    assert!(2 + 2 == 4); // 使用assert 断言判断左右两个参数是否想等
    // assert!(2 + 2 == 5, "这个断言语句不相等"); // 一个不相等的断言语句
}

#[test]
#[should_panic(expected = "断言失败")] // 预期该单元测试应当发生 panic ,并且 期望的panic 中应包含 指定的文本内容，如果不满足条件则会测试失败
fn a_unit_test_with_should_panic() {

    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 5, "断言失败，左值：{}, 右值: {}", 2 +2, 5);
}

#[test]
fn use_result_as_test_return() -> Result<(), String> { //使用 Result 作为返回值的 test 不能加 #[should_panic] 属性，因为这个测试永远不会返回 panic
    if 2 + 2  == 4 {
        Ok(())
    } else {
        Err(String::from("use result as test return get an err result"))
    }
}

#[test]
#[ignore] // 加了这个属性之后，这个测试默认在运行 cargo test 命令时，不会被执行，如果要执行加了这个属性的测试， 需要使用 cargo test -- --ignored 命令
fn a_ignore_test_function() {
    println!("This is a ignore test");
    assert_ne!(2 + 2 + 2, 4);
}