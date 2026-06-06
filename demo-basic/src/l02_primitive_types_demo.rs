use std::{fmt, mem};
use std::fs::write;

/**
 * 原生类型
 * rust 提供了多种原生类型
 * 标准原生类型：
 * - 有符号整数： i8、i16、i32、i64、i128 和 isize(指针大小)
 * - 无符号整数： u8、u16、u32、u64、u128 和 usize(指针大小)
 * - 浮点数： f32、f64
 * - char Unicode指标值，如 'a'、'α' 和 '∞' 每个都是四字节
 * - bool 值为 true 或 false
 * - 单元类型 () ，其唯一可能的值是空元组：()，尽管单元类型的值是一个元组，但它不视为符合类型，因为它不包含多个值。
 */
pub fn run () {
    let logical: bool = true;

    let a_float: f64 = 1.8; // 常规标注
    let b_float = 1.8f32; // 常规标注
    let an_integer = 5i32; // 后缀标注
    println!("打印bool值：{}", logical);
    println!("打印浮点数：{}", a_float);
    println!("打印浮点数：{}", b_float);
    println!("打印整数(后缀标注)：{}", an_integer);

    // 直接使用默认类型
    let default_float = 3.0; // f64
    let default_integer = 9; // i32

    let mut inferred_type = 12; // 类型可以根据上下文进行推理，例如下一行代码中的内容可以得出当前变量是i64整数
    println!("打印i64的数据：{}", inferred_type);
    inferred_type = 1274237467121i64; // 使用中重新指定数据类型
    println!("打印i64的数据：{}", inferred_type);


    // 可变变量的值可以改变
    let mut mutable:i32 = 55;
    println!("可变变量修改前的值：{}", mutable);
    mutable = 66;
    println!("可变变量修改后的值：{}", mutable);
    // mutable = true; // 不支持对变量的类型进行改变

    // 变量可以通过遮蔽，来重置覆盖值
    let mutable = true;

    // 声明数组类型，数组的签名由类型 T 和长度组成，表示成 [T; length]
    let array:[i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("打印数组类型：{:?}", array);// 数组类型打印默认没有实现Display，只能通过Debug格式化方式打印

    // 元组是不同类型值的集合，元组使用圆括号()来进行构建
    let my_tuple = (8i32,6u8,55u32,true, -88.90f32, [1,2,4]);
    println!("打印元组类型中所有元素的值：{:?}", my_tuple);
    println!("打印元组中指定元素的值：{}", my_tuple.0);
    println!("打印元组中指定元素的值：{}", my_tuple.1);
    println!("打印元组中指定元素的值：{}", my_tuple.2);
    println!("打印元组中指定元素的值：{}", my_tuple.3);
    println!("打印元组中指定元素的值：{}", my_tuple.4);
    println!("打印元组中指定元素的值：{:?}", my_tuple.5);

    /// 整数1、浮点数1.2、字符'a'、字符串"abc"、布尔值true和单元类型()可以用字面值表示
    /// 整数也可以用二进制、八进制、十六进制表示法，分别使用这些前缀：0b、0o、0x
    /// 可以在数字中插入下划线以提高可读性，例如：1_000 与 1000 相同，0.000_001 与 0.000001 相同
    /// rust还支持科学计数法E-notation，例如：1e6、7.6e-4。相关类型是f64
    /// 我们需要告诉编译器我们使用的字面值的类型。
    /// 现在，我们将使用u32后缀来表示该字面值是一个无符号32位整数，使用i32后缀来表示一个有符号的32位整数。
    /// rust中的运算符及其优先级与其他类C语言类似：
    ///  分类     运算符
    ///  算术运算   + - * / %
    ///  比较运算   == != > < >= <=
    ///  逻辑运算   && || !
    ///  位运算    & | ^ << >>
    ///  赋值运算   = += -= *= /= %= &= |= ^= <<= >>=
    ///  引用相关   & &mut *
    ///  范围运算   .. ..=
    ///  路径与访问  . :: ? [] ()
    /// -----------------------------------------------------
    /// 优先级从高到低：
    /// 优先级	运算符
    /// 1	() [] .
    /// 2	! - *（解引用） &
    /// 3	* / %
    /// 4	+ -
    /// 5	<< >>
    /// 6	&
    /// 7	^
    /// 8	`
    /// 9	== != < > <= >=
    /// 10	&&
    /// 11	`
    /// 12	.. ..=
    /// 13	= += 等赋值

    // 整数加法
    println!("整数加法：1 + 2 = {}", 1u32 + 2);
    // 整数减法
    println!("整数减法：1 - 2 = {}", 1i32 - 2);
    // println!("1 - 2 = {}", 1u32 - 2); // 这里使用了错误的 u32类型，导致数据溢出，运行报错：attempt to compute `1_u32 - 2_u32`, which would overflow

    // 科学计数法
    println!("科学计数法： 1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // 短路布尔逻辑
    println!("布尔值逻辑计算：true and false is {}", true && false);
    println!("尔值逻辑计算：true or false is {}", true || false);
    println!("尔值逻辑计算：not true is {}", !true);

    // 位运算
    println!("二进制位运算：0011 and 0101 is {:04b}", 0b0011u8 & 0b0101);
    println!("二进制位运算：0011 or  0101 is {:04b}", 0b0011u8 | 0b0101);
    println!("二进制位运算：0011 xor 0101 is {:04b}", 0b0011u8 ^ 0b0101);
    println!("二进制位运算：1 << 5 is {}", 1u8 << 5);
    println!("十六进制位运算：0x80 >> 2 is {:x}", 0x80u8 >> 2);

    // 使用下划线来增加大额数字的可读性
    println!("One million is {}", 1_000_000u32);

    /// # 元组
    /// 元组是一个可以包含各种类型的值的集合，元组使用圆括号()来构造，而且每个元组本身就是一个类型标记为(T1，T2，...)的值，
    /// 其中T1、T2是其成员的类型。函数可以使用元组来返回多个值，因为元组可以存储任意数量的值。



    // 声明一个包含多种不同数据类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64,
                                                    0.1f32, 0.2f64, 'a', true);
    // 可以使用元组索引来取任意元组的值
    println!("多数据类型元组的第1个值：{}", long_tuple.0);
    println!("多数据类型元组的第2个值：{}", long_tuple.1);
    println!("多数据类型元组的第3个值：{}", long_tuple.2);
    println!("多数据类型元组的第4个值：{}", long_tuple.3);
    println!("多数据类型元组的第5个值：{}", long_tuple.4);
    println!("多数据类型元组的第6个值：{}", long_tuple.5);
    println!("多数据类型元组的第7个值：{}", long_tuple.6);
    println!("多数据类型元组的第8个值：{}", long_tuple.7);
    println!("多数据类型元组的第9个值：{}", long_tuple.8);
    println!("多数据类型元组的第10个值：{}", long_tuple.9);
    println!("多数据类型元组的第11个值：{}", long_tuple.10);
    println!("多数据类型元组的第12个值：{}", long_tuple.11);

    // 元组中也可以包含元组成员
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, 8u8), -6i8);
    println!("打印元组：{:?}", tuple_of_tuples);
    // 元组长度超过12个元素，则无法打印
    let too_long_tuple = (1u8, 2u8, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("打印超过12个元素的元组：{:?}", too_long_tuple); // 最多只能为包含 12 个元素的元组设置格式 `(u8, u8, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)` 不实现 `Debug` (`{:?}` 需要): E0277

    let pair = (1, true);
    println!("反转前元组对的值为：{:?}",pair);
    // 调用反转元组对函数
    let reverse_pair = reverse(pair);
    println!("反转后元组对的值为：{:?}", reverse_pair);

    // 创建单元素元组时，需要在结尾使用一个逗号,来区分他们与被括号包围起来的字面量
    let single_element_tuple = (888u32,);
    let single_integer = (888u32);
    println!("打印单元素元组值(888u32,)：{:?}", single_element_tuple);
    println!("打印被圆括号围起来的字面量(888u32)：{:?}", single_integer);

    // 创建一个元组后，可以将元组中的元素解构为单独的变量
    let tuple = (1, "hello", true, "world", 6.6f32, -8.88e-4, "rust");
    let (a, b, c, d, e, f, g) = tuple;
    println!("打印从元组中解构出来的变量值：a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}", a, b, c, d, e, f, g);
    println!("打印从元组中解构出来的变量值：e + f = {}", e + f);
    println!("打印从元组中解构出来的变量值：b + d + g = {}", format!("{} {} {}", b, d, g)); // rust中需要使用format!函数来拼接多个字符串



    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("使用Debug函数打印矩阵：\n{:?}", matrix);
    println!("使用美化Debug函数打印矩阵：\n{:#?}", matrix);
    // 使用自定义实现的Display打印矩阵
    println!("使用自定义矩阵打印函数打印矩阵：\n{}", matrix);
    println!("\n\n");
    // 矩阵转换
    println!("Matrix:");
    println!("{}", matrix);
    println!();
    println!("Transpose:");
    println!("{}", transpose(matrix));

    /// 数组和分片
    /// 数组是一种存储在连续内存中的相同类型T的对象集合。
    /// 数组使用方括号[]创建，其长度在编译时已知，是其签名[T; length]的一部分。
    /// 切片类似于数组，但长度在编译时未知。
    /// 切片是一个双字对象：第一个字是指向对象的指针，第二个字是切片的长度。
    /// 字的大小和usize相同，由处理器架构决定，例如在x86-64上是64位。
    /// 切片可用于借用数组的一部分，其类型签名为 &[T]。

    // 固定大小的数组(类型签名是多余的)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有的元素可以初始化为相同的值
    let ys: [i32; 500] = [0; 500];

    println!("打印数组的第一个元素：{}", xs[0]);
    println!("打印数组的第二个元素：{}", xs[1]);

    println!("打印固定大小数组的全部元素：{:#?}", xs);
    println!("打印相同值初始化数组的全部元素：{:?}", ys);
    println!("打印数组的长度：ys长度为：{}", ys.len());
    println!("打印数组的长度：xs长度为：{}", xs.len());
    // 打印数组占用的空间
    println!("打印数组占用的内存空间：{}", mem::size_of_val(&xs));
    println!("打印数组占用的内存空间：{}", mem::size_of_val(&ys));

    // 将整个数组借用为切片
    println!("将整个数组借用为切片");
    analyze_slice(&xs);
    analyze_slice(&ys);

    // 切片可以指向数组的一部分
    // 它的形式是 [起始索引 .. 结束索引]，取之范围是[起始索引, 结束索引)，注意：起始索引包含，结束索引不包含
    println!("借用数组中的一部分作为切片");
    analyze_slice(&xs[1 .. 4]);

    // 定义一个空数组
    let empty_array:[u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // 可以使用 '.get(i)'安全地访问数组，它返回一个Option
    // 可以像下面这样对其进行匹配，或者使用'.expect()'
    // 如果你希望程序在访问越界时，优雅地退出而不是继续执行
    // 可以使用 '.expect()'
    for i in 0..xs.len()+1 { // 糟糕，访问超出了数组范围
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("慢着！{}超出范围了！", i),
        }
    }

    // println!("{}", xs[5]);// 直接访问越界下标时，会报编译错误
    // println!("{}", xs[..][5]);// 对切片的越界索引，会导致运行错误


}

// 元组可以用作函数参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' 可以用来把元组的成员绑定到变量
    let (integer, boolean) = pair;
    (boolean, integer)
}

// 创建一个元组矩阵
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// 为Matrix实现Display格式化输出
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 实现方式一
        // write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
        // 实现方式二
        writeln!(f, "( {}, {} )",self.0, self.1)?; // writeln! 在打印后自动换行
        write!(f, "( {}, {} )", self.2, self.3)
    }
}

// 二维矩阵行列转换
fn transpose(matrix: Matrix) -> Matrix {
    // 结构元组
    let Matrix(a, b, c, d) = matrix;
    // 返回转换后的矩阵
    Matrix(a, c, b, d)
}

// 次函数借用一个切片
fn analyze_slice(slice: &[i32]) {
    println!("切片的第一个元素：{}", slice[0]);
    println!("切片有{}个元素", slice.len());
}