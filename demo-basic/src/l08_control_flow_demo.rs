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


}