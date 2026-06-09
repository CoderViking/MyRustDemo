/// # 控制流
/// ## if/else
/// if-else 分支结构与其他语言类似。
/// 不同之处在于，布尔条件不需要用括号括起来，每个条件后面都跟着一个代码块。
/// if-else 条件是表达式，所有分支必须返回相同的类型
/// ## loop
/// Rust 提供 loop 关键字来表示无限循环
/// break 语句可以随时退出循环，而 continue 语句可以跳过当前迭代的剩余部分并开始下一次迭代。
/// ## 嵌套和标签
/// 在处理嵌套循环时，可以 break 或 continue 外层循环。
/// 这种情况下，循环必须用 label 标记， 而且必须将标签传递给 break/continue 语句。
/// ## 在 loop 中返回值
/// loop 的一个作用是重试操作直到成功。
/// 如果一个操作返回一个值，你可能需要将它传递给代码的其余部分：
///     将它放在 break 之后，他将被 loop 表达式返回
/// ## while
/// while 关键字用于在条件为真时运行循环。
/// 让我们用 while 循环来编写著名的 FizzBuzz 程序。
/// ## for 循环
/// ### for 和 range
/// for in 结构可用于遍历 Iterator。
/// 创建迭代器最简单的方式之一就是使用区间表示法 a..b。这会生成从a(包含)到b(不包含)的值，步长为1。
/// ## for 与迭代器
/// for in 结构能以多种方式与 Iterator 交互。
/// 正如 Iterator 特质一节讨论的那样，默认情况下 for 循环会对集合应用 into_iter 函数。
/// 然而，这并不是将集合转换为迭代器的唯一方法。
/// into_iter、iter 和 iter_mut 都以不同的方式处理集合到迭代器的转换，通过提供对数据的不同视图。
/// - iter 在每次迭代中借用集合的每个元素。因此，集合保持不变，并且在循环之后可以重复使用。
/// - into_iter 这会消耗集合，使得在每次迭代中提供确切的数据。一旦集合被消耗，他就不再可用于重复使用，因为它已经在循环中被 “移动”了。
/// - iter_mut 这会可变地借用集合的每个元素，允许再原地修改集合。
/// ## match
/// Rust 通过 match 关键字提供模式匹配，类似于 C 语言的 switch。
/// 第一个匹配的分支会被求值，并且必须覆盖所有可能的值。
/// ### 解构
/// match 块可以以多种方式解构项
/// - 元组
/// - 数组/切片
/// - 枚举
/// - 指针/引用
///     - 对于指针，需要区分解构和解引用，因为它们是不同的概念
///         - 解构引用使用 *
///         - 解构使用 & 、ref 和 ref mut
/// - 结构体
pub fn run () {

    let n = 500;

    if n < 0 {
        print!("{} 是负数", n);
    } else if n > 0 {
        print!("{} 是正数", n);
    } else {
        print!("{} 是零", 0);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!("，是一个小数字，扩大十倍");
            // 这个表达式返回一个 i32 类型
            10 * n
        } else {
            println!(", 是一个大数字，将数字减半");
            // 这个表达式也必须返回一个 i32 类型
            n /2
        };

    println!("{} -> {}", n, big_n);

    let mut count:u32 = 0;

    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过本次迭代的剩余部分
            continue;
        }
        println!("{}", count);

        if count == 50 {
            println!("好了， 够了，循环到此为止");
            // 退出整个循环
            break;
        }
    }


    'outer: loop {
        println!("进入外层循环...");
        'inner: loop {
            println!("进入内层循环...");
            // 这个中断智慧中断内存循环
            // break;
            // 这个中断会中断外层循环
            break 'outer;
        }
        println!("这一点永远不会到达");
    }
    println!("退出外层循环");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);
    assert_eq!(result, 20);

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0{
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Rust 中没有 n++ 这类运算，因此只能使用 n+=1; 来自增，实现计数器递增
        n+=1;
    }

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 可以使用 a..=b 表示两端都包含的范围。
    for n in 1..=100 {
        if n % 15 == 0 {
            print!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // iter 在每次迭代中借用集合的每个元素，因此，集合保持不变，并且在循环之后可以重复使用。
    let names = vec!["张三", "李四", "王五"];

    for name in names.iter() {
        match name {
            &"王五" => println!("这里有一个王老五"),
            _ => println!("你好 {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter 这会消耗集合，使得在每次迭代中提供确切的数据。
    // 一旦集合被消耗，他就不再可用于重复使用，因为已经在循环中被 “移动”了。

    for name in names.into_iter() {
        match name {
            "王五" => println!("这里有一个王老五"),
            _ => println!("你好 {}", name),
        }
    }
    // names 已经被移动了，无法再被使用
    // println!("names: {:?}", names);


    let mut names = vec!["张三", "李四", "王五"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "王五" => "这里有一个王老五",
            _ => "你好",
        }
    }
    println!("names: {:?}", names);

    // 在上面的代码片段中，注意 match 分支的类型，这是迭代类型的关键区别。
    // 类型的差异意味着可以执行不同的操作。


    let number = 1;

    println!("告诉我关于 {} 的信息", number);
    match number {
        // 匹配单个值
        1 => println!("一!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("这是质数"),
        // 匹配一个区间范围
        13 .. 19 => println!("一个青少年"),
        // 处理默认情况，match 语句中必须要有这个语句
        _ => println!("没什么特别的"),
    }

    let boolean = true;
    let binary = match boolean {
        // match 的分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    // 元组可以在 match 中按如下方式解构：

    // let triple = (0, -2, 4);
    let triple = (1, -2, 2);

    println!("告诉我关于 {:?} 的一切", triple);

    // match 可用于解构元组
    match triple {
        // 解构第二和第三个元组
        (0, y, z) => println!("第一个是: {}, y: {}, z: {}", 0, y, z),
        (1, ..) => println!("第一个是 1, 其余的不重要"),
        (.., 2) => println!("最后一个是2，其余的不重要"),
        (3, .., 4) => println!("第一个是3， 最后一个是4， 其余的不重要"),
        // .. 可用于忽略元组中的其余部分
        _ => println!("它们是什么并不重要"),
        // _ 表示不绑定值到变量
    }
    // 与元组类似，数组和切片也可以用这种方式解构：

    let array = [-1, -2, 9];
    println!("数组 array 的全部内容为: {:?}", array);

    match array {
        // 将第二和第三个元素分别绑定到相应的变量
        [0, second, third] => println!("array[0] = 0，array[1]  = {}, array[2] = {}", second, third),
        // 单个值可以用 _ 忽略
        [1, _, third] => println!("array[0] = 1, array[2] = {}, array[1] 被忽略了", third),
        // 多个值可以用 .. 忽略
        [-1, second, ..] => println!("array[0] = -1, array[2] = {}, 其余的被忽略了", second),
        // 下面的代码无法编译
        // [-1, second] => println!("这段代码无法编译"),
        // 将他们存储在另一个数组/切片中(类型取决于正在匹配的值的类型)
        [3, second, tail@ ..] => println!("array[0] = 3, array[1] = {}, 其余元素是{:?}", second, tail),
        // 结合这些模式，我们可以，例如，绑定第一个和最后一个值，并将其与的存储在一个单独的数组中
        [first, middle @ .., last] => println!("array[0] = {}, 中间部分 = {:?}, array[2] = {}", first, middle, last),
    }

    // enum 枚举的解构方式类似
    #[allow(dead_code)]
    enum Color {
        // 这三个仅通过名称指定
        Red,
        Blue,
        Green,
        // 这些同样将 u32 元组与不同的名称(颜色模型)关联
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::CMY(122, 17, 40);
    let color = Color::Red;
    let color = Color::HSL(252, 117, 240);
    println!("这是什么颜色？");
    // 可以使用 match 来解构 enum
    match color {
        Color::Red => println!("颜色是红色！"),
        Color::Blue => println!("颜色是蓝色！"),
        Color::Green => println!("颜色是绿色！"),
        Color::RGB(r, g, b) => println!("红: {}, 绿: {}, 蓝: {}！", r, g, b),
        Color::HSV(h, s, v) => println!("色相: {}, 饱和度: {}, 明度: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("色相: {}, 饱和度: {}, 亮度: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("青: {}, 品红: {}, 黄: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("青: {}, 品红: {}, 黄: {}, 黑: {}!", c, m, y, k),
        // 不需要其他分支，因为所有变体都以检查
    }

    // 对于指针，需要区分解构和解引用，因为它们是不同的概念，其用法与 C/C++ 等语言不同。
    // - 解构引用使用 *
    // - 解构使用 & 、ref 和 ref mut

    // 分配一个 i32 类型的引用，用 & 表示
    // 正在分配一个引用
    let reference = &500;

    match reference {
        // 如果 reference 与 &val 进行模式匹配，结果就像这样：
        // &i32
        // &val
        // 我们可以看到，如果去掉匹配的 & 那么， i32 应该被赋值给 val
        &val => println!("通过解构获得的值: {:?}", val),
    }

    // 为了避免 & ，你可以在匹配前解引用
    match *reference {
        val => println!("通过解引用获得的值: {:?}", val),
    }

    // 如果一开始就没有引用怎么办？ reference 是一个 & 因为右侧已经是一个引用。
    // 这不是一个引用，因为右侧不是引用。
    let _not_a_reference = 300;

    // Rust 提供 ref 正式为了这个目的，它修改了赋值，为元素创建了一个引用；
    // 这个引用被赋值
    let ref _is_a_reference = 8000;

    // 相应的，通过定义两个没有引用的值，可以通过 ref 和 ref mut 获取引用
    let value = 5;
    let mut mut_value = 7;

    // 使用 ref 关键字创建引用
    match value {
        ref r => println!("r = {}", r),
    }

    // 类似地使用 ref mut
    match mut_value {
        ref mut m => {
            // 获得了一个引用，在我们能够对其进行任何添加操作之前，必须先解引用
            *m += 10;
            println!("我们加了10， mut_value = {}", m);
        }
    }
    // 同样，结构体也可以按照如下方式解构
    #[derive(Debug)]
    struct Foo{
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (3, 2), y: 3 };
    println!("foo = {:?}", foo);

    match  foo {
        Foo {x:(1, b), y} => println!("x 的第一个元素是：1, b = {}, y = {}", b, y),
        // 可以解构结构体并重命名变量
        // 变量的顺序不重要
        Foo {y :2, x:i} => println!("y 为 2, i = {:?}", i),
        // 可以用 .. 忽略某些字段
        Foo {y, ..} => println!("y = {}, x 的值我们不关心", y),

        // 这会导致编译错误，因为模式中未提及变量 x
        // Foo{ y} => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };
    // 解构结构体不一定需要 match 块
    let Foo{x:x0,y:y0} = faa;
    println!("外部：x0 = {x0:?}, y0 = {y0}");

    // 解构也适合嵌套结构体
    #[derive(Debug)]
    struct Bar {
        foo: Foo,
    }

    let bar = Bar {foo: faa};
    println!("bar = {:?}", bar);
    // 解构嵌套结构体
    let Bar {foo:Foo {x: nested_x, y: nested_y}} = bar;
    println!("嵌套：nested_x = {nested_x:?}, nested_y = {nested_y:?},");

    // # 守卫
    // match 分支可以使用守卫进行额外的筛选

    enum Temperature {
        Celsius(f32),
        Fahrenheit(f32),
    }

    let temperature = Temperature::Celsius(32.0);
    let temperature = Temperature::Fahrenheit(92.0);

    match temperature {
        Temperature::Celsius(t) if t > 30.0 => println!("{}°C高于 30°C", t),
        Temperature::Celsius(t) => println!("{}°C 不高于30°C", t),
        Temperature::Fahrenheit(t) if t > 86.0 => println!("{}°F 高于 86°F", t),
        Temperature::Fahrenheit(t) => println!("{}°F 不高于 86°F", t),
    }

    // let number:u8 = 4 * 80; // 运行错误：边界溢出
    let number:u8 = 80;

    match number {
        i if i == 0 => println!("零"),
        i if i > 0 => println!("大于零"),
        // 正常情况下，上面两种情况已经覆盖了 u8 类型的所有有效范围，但 rust 强制要求必须添加如下的意外情况模式语句，否则编译错误
        _ => unreachable!("不应该出现的情况")
    }

    // # 绑定
    // 间接访问变量时，无法在分支中使用该变量而不重新绑定。
    // match 提供了 @ 符号，用于将值绑定到名称：

    println!("告诉我你是什么类型的人");

    match  age() {
        0 => println!("我还没过第一个生日"),
        // 直接使用  match 1..=12，这时判断匹配哪个模式，但无法确定具体的值是多少，
        // 使用 if 守卫条件，则无法遍历所有的可能，
        // 因此，使用 @ 符号将匹配的值绑定到变量 n上，就可以知道具体的值是多少了。
        n @ 1 ..=12 => println!("我是{:?}岁的儿童", n),
        n @ 13 ..=17 => println!("我是{:?}岁的青少年", n),
        n @ (1 | 7 | 13 | 15) => println!("我是{:?}岁的青少年", n),
        // 没有绑定，直接返回结果
        n => println!("我是{:?}岁的成年人", n),
    }
    // 也可以用绑定来解构 enum 变体，例如：Option
    match some_number() {
        // 匹配确定数字
        Some(n @ 52) => println!("答案是: {} ！", n),
        // 匹配其他数字
        Some(n) => println!("答案不是: {}", n),
        // 其他任何情况
        _ => println!("默认的其他情况"),
    }

    // if let
    // 在某些情况下， 使用 match 匹配枚举会显得繁琐。例如：
    let optional = Some(8);
    
    match optional {
        Some(i ) => println!("这是一个很长的字符串，其中包括 {:?}",i),
        // 这是必须的， 因为 match 要求穷举所有情况
        _ => {}
    }

    // 以下都是Option<i32>类型
    let number = Some(8);
    let letter: Option<i32> = None;
    let emoji: Option<i32> = None;

    // if let 结构的含义是，如果 let 能够将 number 解构为 Some(i)，则执行代码块 {}
    if let Some(i) = number {
        println!("匹配到: {:?}", i);
    }

    // 如果需要指定匹配失败的场景，可以使用 else
    if let Some(i) = letter {
        println!("匹配到: {:?}", i);
    } else {
        // 解构失败，转到失败处理的场景
        println!("没有匹配到数字，那就用一个字母吧！");
    }

    // 童工一个修改后的失败条件
    let i_like_letters = false;

    if let Some(i) = emoji {
        println!("匹配到: {:?}", i);
        //解构失败，评估 else if 条件，看是否应执行替代的失败分支
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }
    // 同样的，if let 可以用来匹配任何枚举值

    enum Boo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = Boo::Baz;
    let b = Boo::Baz;
    let c = Boo::Qux(100);

    // 变量 a 匹配 Boo::Bar
    if let Boo::Bar = a {
        println!("a 是 foobar");
    }

    // 变量 b 不匹配 Boo::Bar
    // 所以这里不会打印任何内容
    if let Boo::Bar = b {
        println!("b 是 foobar");
    }

    // 变量 c 匹配 Boo::Qux, 它包含一个值，类似于前面例子中的 Some()
    if let Boo::Qux(value) = c {
        println!("c 是 {}", value);
    }

    // if let 也可以进行绑定
    if let Boo::Qux(x @ 100) = c {
        println!("c 是一百");
    }

    // 挑战练习
    enum Coo{
        Bar
    }

    let a = Coo::Bar;
    // 如果不加 let 直接使用 == 判读是否相等，则会编译错误：二元运算，
    // if Coo::Bar == a {
    // 改为使用 if let 组合，成功匹配
    if let Coo::Bar = a {
        println!("a 是 coo_bar");
    }

    // # let-else
    // let-else语法允许可能失败的模式匹配像普通let一样绑定到变量当前的作用域，或在匹配失败时中断操作（如：break、return、panic!）


    fn get_count_item(s:&str) -> (u64, &str){
        let mut it = s.split(' ');
        // let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        //     panic!("无法分割计数项对：'{s}'");
        // };
        //
        // let Ok(count) = u64::from_str(count_str) else {
        //     panic!("无法解析整数：'{count_str}'");
        // };
        // (count, item)

        // 名称绑定的区别是使其区别于 match 或 if let-else 表达式的主要特点。
        // 在此之前，可能需要通过一些荣誉的重复和外部 let 来近似实现这些模式
        let (count_str, item) = match (it.next(), it.next()) {
            (Some(count_str), Some(item)) => (count_str, item),
            _ => panic!("无法分割计数项对: '{s}'"),
        };
        let count = if let Ok(count) = u64::from_str(count_str) {
            count
        } else {
            panic!("无法解析整数: '{count_str}'");
        };
        (count, item)
    }

    println!("执行结果：{:?}", get_count_item("3 chairs"));
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // ## while let
    // 与 if let 类似，while let 可以简化繁琐的 match 序列。
    // 以下面的递增 i 为例：

    // 创建 Option<i32> 类型的 optional
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i ) => {
                if i > 9 {
                    println!("大于9，退出！");
                    optional = None;
                } else {
                    println!("'i' 是 '{i:?}', 再试一次。");
                    optional = Some(i + 1);
                }
            },
            _ => {
                break;
            }
        }
    }

    // 使用 while 可以让这个序列更简洁
    let mut optional = Some(0);
    // 这段代码的含义是： 当 let 将 optional 解构为 Some(i) 时，执行代码块 {} ，否则 break
    while let Some(i) = optional {
        if i > 9 {
            println!("大于9，退出！");
            optional = None;
        } else {
            println!("'i' 是 '{i:?}', 再试一次。");
            optional = Some(i + 1);
        }
        // 这里减少了代码缩进，无需显式处理失败的情况
        // 这里 while let 没有额外的 else/else if 子句，if let 是可以有的
    }


}
use std::str::FromStr;
fn age() -> u8 {
    0
}
fn some_number() -> Option<u32> {
    Some(520)
}