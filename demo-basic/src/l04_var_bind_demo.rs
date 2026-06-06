/// # 变量绑定
/// Rust 通过静态类型提供类型安全，变量绑定在声明时可以添加类型注解。
/// 然而大多数情况下，编译器能够从上下文中推断出变量类型，大大减少了类型的注解的负担
/// 可以使用 let 关键字将值(如字面量)绑定到变量

pub fn run() {
    let an_integer = 7u32;
    let a_boolean = true;
    let unit = ();

    // 将 an_integer 的值复制给 copied_integer
    let copied_integer = an_integer;

    println!("整数：{}", an_integer);
    println!("布尔值: {}", a_boolean);
    println!("单元值: {:?}", unit);

    // 编译器会对未使用的变量绑定发出警告
    // 可以通过在变量名称前加下划线_来消除这种警告
    let _unused_variable = 32u32;

    // 未使用的变量，编译器会发出警告
    let noisy_unused_variable = 66u32;

    /**
     * 可变性
     * 变量绑定默认是不可变的，可以使用 mut 修饰符来改变这一行为。
     */
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("修改前: {}", mutable_binding);
    mutable_binding += 1; // 自增运算
    mutable_binding <<= 5; // 左移运算

    println!("修改后: {}", mutable_binding);

    // _immutable_binding = 2; // 编译错误，不能给不可变变量赋新值
    /**
     * 变量绑定有作用域，它们被限制在一个代码块中生存。代码块是由花括号{}包围的一系列语句
     */
    // 这个绑定作用于 run 函数中
    let long_lived_binding = 1;
    // 这是一个代码块，它的作用域比run函数小
    {
        let short_lived_binding = 2;
        println!("内部 short: {}", short_lived_binding);
        println!("内部代码块可以访问外部代码块的变量: long_lived_binding: {}", long_lived_binding);
    }
    // 错误，short_lived_binding 在此作用域中不存在
    // println!("外部 short: {}", short_lived_binding);

    println!("外部 short: {}", long_lived_binding);

    /**
     * 作用域和遮蔽
     * rust允许变量遮蔽，变量遮蔽之后，变量相当于在之后就是一个新的变量了
     */
    let shadowed_binding = 10;
    {
        println!("被遮蔽前，变量值：{}", shadowed_binding);
        let shadowed_binding = "Hello Rust";
        println!("内部代码块中被遮蔽: {}", shadowed_binding);
    }

    println!("内部代码块外: {}", shadowed_binding);

    // 这里的变量绑定遮蔽了之前的绑定
    let shadowed_binding = false;

    println!("外部代码块中被遮蔽: {}", shadowed_binding);

    /**
     * 先声明
     * 可以看声明变量绑定然后再初始化，但所有变量绑定在使用之前必须初始化，编译器禁止使用未初始化的变量绑定，因为这会导致未定义行为。
     * 在函数中先声明变量绑定而稍后再初始化的做法并不常见。当初始化与声明分离时，读者更难找到初始化的位置。
     * 更常见的做法是在变量即将被使用的地方附近声明并初始化
     */
    // 先声明一个变量绑定
    let a_binding;
    {
        let x = 66.88;
        // 初始化绑定
        a_binding = x * 20.5;
    }
    println!("绑定值：{}", a_binding);

    let another_binding;

    // 错误：使用未初始化的绑定
    // println!("另一个绑定值: {}", another_binding);

    another_binding = 1_000;
    println!("另一个绑定值: {}", another_binding);

    /**
     * 冻结
     * 当数据以相同名称被不可变地绑定时，它也会冻结。被冻结的数据，在不可变绑定离开作用域之前不能被修改；
     */

    let mut _mutable_float = 8888.888f32;
    {
        // 通过不可变的 _mutable_float 进行遮蔽
        let _mutable_float = _mutable_float;
        // 此时变量在遮蔽的作用域内被冻结
        // _mutable_float = 6000.0f32;
    }
    _mutable_float = 999.999;
    println!("变量值: {}", _mutable_float);

    // 变量未声明，无法直接使用后
    // println!("打印KKK: {}", kkk);


    let kkk = 20;

    let Y(y1, y2) = Y(888, 99);
    println!("yyy: ({}, {})", y1, y2);
    #[derive(Debug)]
    struct Y(u32,u32);
}

