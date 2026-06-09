/// # 函数
/// 函数使用 fn 关键字声明，函数参数需要标注类型，就像变量一样。如果函数有返回值，则必须在箭头 -> 后指定返回类型.
/// 函数的最后一个表达式将作为返回值。另外，可以使用 return 语句在函数内部提前返回值，甚至可以在循环或 if 语句内部使用。
/// ## 关联函数和方法
/// 某些函数与特定类型相关联，这些函数有两种形式： 关联函数和方法。关联函数时在类型上定义的函数，而方法时在类型的特定实例上调用的关联函数。
/// ## 闭包
/// 闭包是可以捕获周围环境的函数。例如，下面是一个捕获变量 x 的闭包：
/// |val| val + x
/// 闭包的语法和功能使其非常适合即使使用。调用闭包与调用函数完全相同。不过，闭包的输入和输出类型可以被推断，而输入变量名必须指定。
/// 闭包的其他特点包括：
/// - 使用 || 而不是 () 来包围输入变量
/// - 单行表达式可省略函数体界定符({})，其他情况则必须使用
/// - 能够捕获外部环境的变量
///

pub fn run () {
    // 主要调用函数
    fizz_buzz_to(91);

    let rectangle = Rectangle {
        // 使用双冒号调用函数
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("矩形周长: {}", rectangle.perimeter());
    println!("矩形面积: {}", rectangle.area());

    // 定义一个可变的矩形，用于调用 translate 函数
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    println!("移动前的矩形：{square:?}");

    // rectangle 没有 mut 修饰，属于不可变量，所以无法调用变化方法
    // rectangle.translate(5.0, 5.0);
    square.translate(5.0,5.0);

    println!("移动后的矩形：{square:?}");

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // pair.destroy(); // 编译错误：之前 destroy 调用梯井消耗了 pair,因此无法再使用pair

    let outer_var = 50;

    // 常规函数无法引用外部环境的变量，编译器会建议我们定义一个闭包来替代
    // fn function(i: i32) -> i32 {
    //     i + outer_var
    // }

    // 闭包是匿名的，我们将它们绑定到引用
    // 注解与函数注解相同，但是是可选的
    // 包裹函数的 {} 也是可选的
    // 这些匿名函数被赋值给适当命名的变量
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // 闭包的参数和返回值类型是可以被推断的，可以直接省略，但是参数名必须要有
    let closure_inferred = |i| i + outer_var;

    // 调用闭包
    println!("closure_annotated: {}", closure_annotated(100));
    println!("closure_inferred: {}", closure_inferred(1000));
    // 不能用其他类型重用 closure_inferred， 因为参数的类型一旦被推断，就不能再用其他类型重新推断了
    // println!("closure_inferred: {}", closure_inferred(100u8));

    // 一个无参数并返回 i32 的闭包
    // 返回值是可以推断的
    let one = || 1; // 默认情况下 one 的返回值数据类型是 i32
    let two = one() + 1u8; // 通过这里的代码，推断出了 one 的返回值类型是 u8
    // let three = one() + 2u16; // 这里就不能再调用 one 用于处理 u16 类型的数据了， 因为前面一句已经推断出闭包 one 的返回值类型是 u8
    println!("返回 1 的闭包: {}", one());
    println!("返回 1 的闭包加一个其他类型数字的值：{}", two);
}
// 定义其他被调用函数
// 返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 特殊情况，提前返回
    if rhs == 0 {
        return  false;
    }
    // 这是一个表达式，此处不需要 return 关键字
    lhs % rhs == 0
}

// 无返回值 的函数实际上返回单元类型 ()
fn fizz_buzz (n : u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n ,3) {
        println!("buzz");
    } else if is_divisible_by(n, 5) {
        println!("fizz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 () 时，可以在函数签名中省略返回值类型

fn fizz_buzz_to (n : u32) -> () {
    for n in 1..=n {
        println!();
        fizz_buzz(n)
    }
}

// 关联函数
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 实现结构体，所有 Point 的关联函数和方法都在此处定义
impl Point {
    // 这是一个关联函数，因为这个函数与特定类型 Point 相关联
    // 关联函数不需要通过实例来调用，这些函数通常用作构造函数
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // 另一个接受两个参数的关联函数
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}
// 定义有两个顶点的矩形类型
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// 实现矩形类型，并创建关联函数

impl Rectangle {
    // 这是一个方法
    // &self 是 self: &Self 的语法糖，其中 self 是调用者对象的类型。
    // 在这个例子中 Self = Rectangle
    fn area(&self) -> f32 {
        // self 通过点运算符访问结构体字段
        let Point{ x: x1, y: y1 } = self.p1;
        let Point{ x: x2, y: y2 } = self.p2;
        // abs 是 f32 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f32 {
        let Point{ x: x1, y: y1 } = self.p1;
        let Point{ x: x2, y: y2 } = self.p2;
        // 矩形周长：(长 + 宽) * 2
        ((x1 - x2).abs() + (y1 - y2).abs()) * 2.0
    }

    // 这个方法要求调用对象是可变的
    // &mut self 是 self: &mut Self 的语法糖
    fn translate(& mut self, x: f32, y: f32) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair 拥有两个堆分配的整数资源
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会消耗调用对象的资源
    // self 是 self: Self 的语法糖
    fn destroy(self) {
        // 解构 self
        let Pair(first, second) = self;
        println!("正在销毁 Pair({first}, {second})");
        // first 和 second 超出作用域将被释放
    }
}
