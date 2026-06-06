/// # 类型
/// Rust提供了几种机制来更改或定义原生类型和用户自定义类型，以下部分将介绍：
/// - 原生类型之间的类型转换
/// - 字面量的类型
/// - 使用类型推断
/// - 类型别名

pub fn run() {

    /**
     * 类型转换
     * Rust不支持原始类型之间的隐式类型转换(强制转换)，但可以使用 as 关键词进行显式类型转换。
     * 整数类型之间的转换规则通常遵循C语言惯例，但C中存在未定义行为的情况除外。
     * 在Rust中所有整数类型之间的转换都有明确定义。
     */
    let decimal = 65.4321_f32;
    println!("decimal原始值: {}", decimal);
    // 错误，不允许转换
    // let integer_u8 :u8 = decimal;

    // 显式转换
    let integer_u8 = decimal as u8;
    println!("decimal转换为u8类型之后的值: {}", integer_u8);

    let character = integer_u8 as char;
    println!("decimal转换为u8类型之后再转为char类型的值: {}", character);

    // 错误：浮点数无法直接转换为字符类型
    // let char1 = decimal as char;

    println!("类型转换: {} -> {} -> {}", decimal, integer_u8, character);

    // 1000 在u16的有效值范围内
    println!("1000 转换为 u16 为: {}", 1000 as u16);
    // 1000 -256 - 256 - 256 = 232
    // 实际上保留了最低有效位的前8位，而朝最高有效位方向的其余位被截断。
    println!("1000 转换为 u8 是: {}", 1000u16 as u8);

    // -1 不在u8的有效值范围内, -1 + 256 = 255
    println!("-1 转换为 u8 是: {}", -1i8 as u8);

    // 对于正数，这等同于取模运算
    println!("1000 对 256 取模为: {}", 1000 % 256);

    // 当转换为有符号类型时时，(按位)结果等同于先转换为对应的无符号类型。如果该值的最高有效位为1，则该值为负数。

    // 如果已经在有效值范围内的数据，则不需要进行转换
    println!("128转换为 i8 是: {}", 128i32 as i16);
    // 边界情况：128 在 8 位二进制补码表示中为 -128
    println!("128转换为 i8 是: {}", 128i32 as i8);

    // 232 在有符号八位二进制中的补码为 -24
    println!("232 转位 i8 为: {}", 232i32 as i8);

    // 从Rust 1.45开始， as 关键字在浮点数转换整数时执行 饱和转换
    // 如果浮点值超出上界或低于下界，返回值将等于所越过的边界值
    // 300.0转换为 u8 是 255
    println!("300.0 转换为 u8 为: {}", 300.0 as u8);
    // -100.0 转换为 u8 是 0
    println!("-100.0 转换为 u8 为: {}", -100.0 as u8);
    // NAN 转换为 u8 是 0
    println!(" NaN 转换为 u8 是: {}", f32::NAN as u8);

    // 这种行为会产生少量运行时开销，可以通过不安全方法避免，
    // 但结果可能溢出并返回**不可靠的值**。请谨慎使用这些方法：
    unsafe {
        println!("=========================== unsafe 操作 ===========================");
        // 300.0 转换为 u8 是 44
        println!("300.0 转换为 u8 是: {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 转换为 u8 是156
        println!("-100.0 转换为 u8 是: {}", (-100.0_f32).to_int_unchecked::<u8>());
        // NaN 转换为 u8 是 0
        println!("NaN 转换为 u8 是: {}", f32::NAN.to_int_unchecked::<u8>());
    }


    /**
     * 字面量
     * 数字字面量可以通过添加类型后缀进行类型标注。例如：要指定字面值 66 的字类型为 i32 ,可以写成 66i32
     * 无后缀数字字面量的类型取决于其使用方式。如果没有约束，编译器将对整数使用 i32 ,对浮点数使用 f64
     */

    // 带后缀的字面量，其类型在初始化时确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于使用方式
    let a = 1;
    let b = 1.0;

    // size_of_val 返回变量的字节大小，1个字节8位
    // std::mem::size_of_val 是一个函数，这里使用了它的“完整路径“来调用。代码可以被划分为称为“模块“的逻辑单元。
    // 在这个例子中，size_of_val 函数定义在 mem 模块中，而 mem 模块则定义在 std crate 中。
    println!("x 的字节大小为: {}", std::mem::size_of_val(&x));
    println!("y 的字节大小为: {}", std::mem::size_of_val(&y));
    println!("z 的字节大小为: {}", std::mem::size_of_val(&z));
    println!("a 的字节大小为: {}", std::mem::size_of_val(&a));
    println!("b 的字节大小为: {}", std::mem::size_of_val(&b));


    /**
     * 类型推断
     * 类型推断引擎相当智能。它不仅在初始化时分析值表达式的类型，还会更具变量后续的使用方式来推断其类型。
     */

    // 通过类型注解，编译器得知 elem 的类型为 u8
    let elem = 5u8;
    // 创建一个空向量，可增长数组
    let mut vec = Vec::new();
    // 此时编译器还不知道 vec 的具体类型
    // 只知道它是某种类型的向量

    // 向 vec 向量中插入元素
    vec.push(elem);
    // 现在编译器知道 vec 是 u8 类型的向量(Vec<u8>)了

    println!("{:?}", vec);

    /**
     * 别名
     * type语句用于为现有类型创建新名称。
     * 类型必须使用 UpperCamelCase(大驼峰)命名，否则编译器会发出警告。
     * 此规则的例外是原始类型，例如：usize、f32、u8、bool等
     */

    // 别名的主要用途是减少重复代码。例如，io::Result<T> 类型是 Result<T, io::Error> 类型的别名。
    // `NanoSecond`、`Inch` 和 `U64` 都是 `u64` 的新名称。
    type NanoSecond = u64;

    type U64 = u64;
    type Inch = u64;

    let nano_second:NanoSecond = 50 as u64;
    let inch:Inch = 20 as U64;

    println!("{} 纳秒 + {} 英寸 = {} 单位？ ", nano_second, inch, nano_second + inch);







}