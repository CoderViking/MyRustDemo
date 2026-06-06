use std::fmt::write;

pub fn run() {
    println!("---------------------- 输出打印函数练习demo ----------------------");

    // `{}`会被替换成任何参数，这些参数会被转换成字符串
    println!("一个月有{}天", 30);

    // 可以使用位置参数。在`{}`中指定一个整数，来决定用那个参数替换，参数编号从0开始
    println!("{0}, 这是 {1}. {1}, 这是 {0}", "Alice", "Bo");

    // 可以直接使用命名参数来控制输出内容
    println!("{subject} {verb} {object}",object="那只懒惰的狗",subject="那只敏捷的棕色狐狸", verb="跳过");

    // 在:后指定格式字符，可以调用不同的格式化方式
    println!("十进制:      {}",88990);
    println!("二进制:      {:b}",88990);
    println!("八进制:      {:o}",88990);
    println!("十六进制:    {:x}",88990);

    // 可以指定宽度来右对齐文本，例如指定宽度为5，则不足五位的默认补空格
    println!("{number:>5}", number=1);
    // 可以指定自定义占位字符，来填充不足的位数，例如使用0，或者使用 - 符号
    println!("{number:0>5}", number=1);
    println!("{number:->5}", number=1);
    // 通过翻转符号来使文本左对齐，右边部分可以使用自定义占位符
    println!("{number:0<5}", number=1);
    println!("{number:*<5}", number=1);

    // 在格式说明符号后添加 '$' 符号，可以使用命名参数
    println!("{number:0>width$}", number=666, width=10);

    // 参数数量错误时，rust会提示语法错误
    // println!("我的名字是{0}， {1}, {0}","邦德");

    // rust中默认只能使用 '{}' 格式化打印实现了 fmt::Display的数据类型
    // 自定义的数据类型默认没有实现 fmt::Display，因此无法直接使用 '{}'进行格式化打印
    #[allow(dead_code)]
    struct Structure(i32);

    // 这里无法编译，因为 'Structure' 没有实现 fmt::Display
    // print!("打印结构体 {}", Structure(3));

    let number: f64 = 3.88;
    let width: usize = 10;
    println!("{number:>width$}");

    // 这个结构体无法通过 'fmt::Display' 或 'fmt::Debug' 打印实现
    struct UnPrintable(i32);

    // 'derive' 属性自动创建，使这个结构体可以用 'fmt::Debug' 打印实现
    #[derive(Debug)]
    struct DebugPrintable(i32);


    // 使用 '{:?}' 打印类似于 '{}'
    println!("一年中有 {:?} 个月", 12);
    println!("{1:?} {0:?} 是这个 {actor:?} 的名字。", "Slater", "Christian", actor="演员");

    // fmt::Debug 确实使其可打印，但牺牲了一些优雅。 Rust 还提供了使用 {:#?} 进行“美化打印“的功能。
    println!("{:?}", DebugPrintable(10));
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Steve";
    let age = 27;
    let pet = Person { name, age };
    // 美化打印
    println!("{:#?}", pet);


    // ================ 格式化打印 ======================
    // 通过 `use` 导入 `fmt` 模块使其可用。
    use std::fmt;

    // 要使用 `{}` 标记，必须为该类型手动实现 `fmt::Display` trait。
    impl fmt::Display for Structure {
        // 这个 trait 要求 `fmt` 方法具有确切的签名。
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 将第一个元素严格写入提供的输出流 `f`。
            // 返回 `fmt::Result`，表示操作是否成功。
            // 注意 `write!` 的语法与 `println!` 非常相似。
            write!(f, "{}",self.0)
        }
    }
    // 通过前面实现的 Display ，使得Structure可以直接使用 '{}'进行格式化打印
    println!("打印结构体 Structure：{}", Structure(5));

    // 定义一个两个参数的结构体。派生 'Debug' 特性，以便与 'Display' 进行对比
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // 为MinMax实现 ‘Display’ 特性
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用 'self.number' 引用每个位置的数据点
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    println!("打印多参数的结构体 MinMax: {}", MinMax(-500, 5000));

    // 定义一个结构体，其字段可命名以便比较
    #[derive(Debug)]
    struct Point2D {
        x: i64,
        y: i64
    }

    // 同样为 ‘Point2D’ 实现 'Display' 特性
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 自定义实现，只显示 x 和 y
            write!(f, "(x: {}, y: {})", self.x, self.y)
        }
    }

    println!("打印 Point2D: {}", Point2D { x: 80, y: -60 });

    println!("比较结构体==========");
    println!("Debug: {:?}", Point2D{x: 666, y: 888});
    println!("Display: {}", Point2D{x: 666, y: 888});

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "real: {} + imag: {}i", self.real, self.imag)
        }
    }

    println!("Debug Complex: {:?}", Complex{real: 66.6, imag: 88.8});
    println!("Display Complex: {}", Complex{real: 55.56, imag: 88.89});

    // 定义一个名为 `List` 的结构体，包含一个 `Vec`。
    #[derive(Debug)]
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 创建对存储在 List 结构中的 Vec 的引用
            let vec = &self.0;
            write!(f, "[")?;
            // 遍历 `vec` 中的 `v`，同时用 `index` 枚举迭代索引。
            for (index, v) in vec.iter().enumerate() {

                // 除第一个元素外，为每个元素添加逗号。
                if index != 0 {
                    write!(f, ", ")?;
                }
                // 打印列表中的元组
                write!(f, " {}: {}", index, v)?;
            }
            // 闭合左括号并返回 fmt::Result 值
            write!(f, "]")
        }
    }

    println!("Debug: {:?}", List(vec![1, 2, 3]));
    println!("Display: {}", List(vec![1, 2, 3, 4, 5, 6, 7, 8]));

    #[derive(Debug)]
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let lat_c= if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    for city in [
        City { name: "Dublin", lat: 50.0, lon: -6.0},
        City { name: "Oslo", lat: 50.0, lon: 10.0 },
        City { name: "Vancouver", lat: 50.0, lon: -12.0 },
    ] {
        println!("{}", city);
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for color in [
        Color { red: 255, green: 0, blue: 0 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ]{
        println!("{:?}", color);
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "RGB ({:0>3}, {:0>3}, {:0>3})", self.red, self.green, self.blue)
        }
    }
    for color in [
        Color { red: 255, green: 0, blue: 0 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ]{
        println!("{}", color);
    }
}