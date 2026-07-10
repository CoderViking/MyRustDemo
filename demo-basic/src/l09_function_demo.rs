use std::mem;

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
///  ### 捕获
/// 闭包本质上很灵活，无需注释就能很根据功能需求自动适应。
/// 这使得捕获可以灵活地适应不同场景，有时移动，有时借用。闭包可以通过一下方式捕获变量：
/// - 通过引用 &T
/// - 通过可变引用 &mut T
/// - 通过值 T
/// 闭包优先通过引用捕获变量，仅在必要时才使用在最底部的捕获方式。
/// ### 作为输入参数
/// Rust通常能自动选择如何捕获变量，无需类型标注。
/// 但在编写函数时，这种模糊性是不允许的。
/// 当将一个闭包作为参数输入时，必须使用特定的 trait 来注解闭包的完整类型。这些trait由闭包对捕获值的处理方式决定。
/// 按限制程度由高到低排列如下：
/// - Fn: 闭包通过引用使用捕获变量(&T)
/// - FnMut: 闭包通过可变引用使用捕获变量的值(&mut T)
/// - FnOnce: 闭包通过值使用捕获的值(T)
/// 编译器会以尽可能最小限制的方式逐个捕获变量
/// 例如：考虑一个注解为 FnOnce 的参数。这表示闭包可能通过 &T、&mut T 或 T 进行捕获，但编译器最终会根据捕获变量在闭包中的使用方式来决定。
/// 这是因为，如果可以移动，那么任何类型的借用也是可能的。注意，反过来并不成立，如果参数被注解为 Fn, 那么通过 &mut T 或 T 捕获是不允许的，但&T是允许的。
/// ### 类型匿名
/// 闭包能简洁地从外部作用域捕获变量。这会有什么影响吗？当然会有。注意观察如何将闭包作为函数参数使用时需要泛型，这是由于闭包的定义方式所决定的。
/// ``` fn apply<F> (f:F) where F: FnOnce() { f(); }
/// ```
/// 当定义一个闭包时，编译器会隐式创建一个新的匿名结构来存储内部捕获的变量，同时通过Fn、FnMut 或 FnOnce这些 trait之一为这个位置类型实现功能。
/// 这个类型被赋给变量并存储，直到被调用。
/// 由于这个新类型时未知类型，在函数中使用时就需要泛型。
/// 然而，一个无界的类型参数<T>仍然会是模糊的，不被允许。因此，通过Fn、FnMut 或 FnOnce 这些trait之一(它实现的)来约束就足以指定其类型
/// ### 输入函数
/// 既然闭包可以作为参数使用，你肯呢个好迹象知道函数是否也可以这样使用。
/// 确实可以！如果你声明一个函数，他接受一个闭包作为参数，那么任何满足该闭包trait玉树的函数都可以作为参数传递。
/// ### 作为输出函数
/// 既然闭包可以作为输入参数，那么将闭包作为输出参数返回应该是可行的。
/// 然而，匿名闭包类型本质上是未知的，因此我们必须使用 impl Trait 来返回它们。
/// 可用于返回闭包的有效trait包括：
/// - Fn
/// - FnMut
/// - FnOnce
/// 此外，必须使用move关键字，它表示所有捕获是按值进行的。
/// 这是必要的，因为任何通过以引用捕获的变量都会在函数推出时被丢弃，从而在闭包中留下无效的引用。
/// ### Iterator::any
/// Iterator::any 是一个函数，它接受一个迭代器作为参数，如果任何元素满足给定的条件，则返回 true，否则返回 false
/// ### 高阶函数
/// rust提供了高阶函数（Higher Order Functions, HOF）。这些函数接受一个或多个函数作为参数，并/或产生一个更有用的函数。
/// HOF和惰性迭代赋予了 rust 函数式编程的特性。
/// ### 发散函数
/// 发散函数永不返回。它们使用 ! 标记，这是一个空类型。


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


    let color = String::from("green");
    // 打印color的闭包，立即借用(&) color 并将借用和闭包存储在 print 变量中。借用状态将持续到 print 最后一次使用
    // println! 只需要不可变引用参数，所以不会施加更多限制
    let print = || println!("{}", color);

    // 使用借用调用闭包
    print();

    // color 可以再次被不可变借用，因为闭包只持有 color 的不可变引用
    let _reborrow = &color;
    print();

    // print 最后一次使用，允许移动或重新借用
    let _color_moved = color;

    let mut count = 0;
    // 增加count的闭包，可以接受 &mut count 或 count，但 &mut count 限制更少，所以选择它。
    // 立即借用count
    // inc 需要 mut ,因为内部存储了 &mut。因此调用闭包会修改count，这需要mut
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 闭包仍然可变借用 count，因为他稍后会被调用
    // let _reborrow = &count; // 如果尝试重新借用，会导致错误
    inc();

    // 前面闭包最后一次使用完了，闭包不再需要借用 &mut count , 因此可以在没有错误的前提下，重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型
    let movable = Box::new(3);

    // mem::drop 需要 T ，所以这里必须通过值获取。可复制类型可以被复制到闭包中，原始值保持不变
    // 不可复制类型必须移动，所以 movable 立即移动到闭包中
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    // consume 消耗了变量，因此只能调用一次
    consume();
    // consume();// 再次调用会报错

    let haystack = vec![1, 2, 3];

    println!("haystack: {:?}", haystack);

    // 在竖线前使用 move 强制闭包获取捕获变量的所有权
    // let contains = move |needle| haystack.contains(needle);

    // 不加 move 则不会捕获变量的所有权
    let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 借用检查器不允许变量被移动后重用
    // println!("haystack: {:?}", haystack); // 无法再访问变量，因为在闭包中已经将元素被移动
    // 如果从闭包签名中移除 move 将导致闭包不可变借用 _haystack_ 变量，因此 _haystack_ 仍可用，
    // 取消注释上面的行就不会报错了

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("我说{}。", greeting);
        farewell.push_str("! ! !");
        println!("然后我喊{}。", farewell);
        println!("现在我可以睡觉了。呼呼");
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
     println!("3的两倍是: {}", apply_to_3(double));

    let x = 7;

    // 将 x 捕获到一个匿名类型中并为其实现 Fn
    // 将其存储在 print 中
    let print = || println!("{}", x);
    apply(print);

    // 定义一个满足 Fn 约束的闭包
    let closure = || println!("我是闭包！");

    call_me(closure);
    call_me(function);
    // 另外需要注意的是：Fn、FnMut 和 FnOnce 这些trait 决定了闭包如何从外部作用域捕获变量。

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 使用 iter() 产生 &i32，解构为 i32
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // vec 使用 into_iter() 产生 i32， 无需解构
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    // iter() 只借用 vec1 及其元素，所以它们可以再次使用
    println!("vec1 的长度: {}", vec1.len());
    println!("vec1 的第一个元素: {}", vec1[0]);
    // into_iter() 会移动 vec2 及其元素，所以它们不能再次使用
    // println!("vec2 的第一个元素是：{}", vec2[0]);
    // println!("vec2 长度：{}", vec2.len());

    //
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    println!("array1 的长度: {}", array1.len());
    // into_iter()对数组不会产生移动，所以它们都能再次被使用
    println!("array1 的第一个元素: {}", array1[0]);
    println!("array2 的长度: {}", array2.len());
    println!("array2 的第一个元素: {}", array2[0]);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("在 vec1 中查找 2 : {:?}", iter.find(|&&x| x == 2));
    println!("在 vec2 中查找 2 : {:?}", into_iter.find(|&x| x == 2));

    println!("查找后 vec1 = :{:?}", vec1);
    // 迭代的时候，数组中的值已经被移动，无法被访问
    // println!("查找后 vec2 = :{:?}", vec2);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("在 array1 中查找 2 : {:?}", array1.iter().find(|&&x| x == 2));
    println!("在 array2 中查找 2 : {:?}", array2.into_iter().find(|&x| x == 2));

    println!("执行查找操作后的 array1 = :{:?}", array1);
    println!("执行查找操作后的 array2 = :{:?}", array2);

    let vec = vec![1, 9, 3, 3, 13, 2];
    // 查找数组中第一个偶数的下标
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // 查找数组中第一个负数的下标
    let index_of_first_negative_number = vec.iter().position(|&x| x < 0);
    assert_eq!(index_of_first_negative_number, None);


    println!("找出所有平方为奇数且小于1000的数字之和");
    let upper = 1000;

    // 命令式方法
    // 声明累加器变量
    let mut acc = 0;
    // 循环，从0、1、2、3,...一直到无穷大
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            // 如果超出最大值，则终止循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数，则累加值
            println!("{} 的平方 {} 是奇数。", n, n_squared);
            acc += n;
        }
    }

    println!("命令式风格: {}", acc);

    // 函数式方法
    let sum: i32 = (0..)
            .take_while(|&n| n * n < upper) // 小于上限
            .filter(|&n| is_odd(n * n)) // 筛选出奇数
            .sum(); // 求和

    println!("函数式风格: {}", sum);

    // 放开下面的注释将会发生错误提示
    // foo();
    // 后面的代码永远不会被执行
    // println!("发散函数被调用");




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

// 这个函数接受一个闭包作为参数，并调用它
// <F> 表示 F 是一个泛型类型参数
fn apply<F> (f: F) where F: FnOnce() { // 这里限定了这个闭包不接受任何参数，也不返回任何值
    f();
}


fn apply_to_3<F> (f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

// 定义一个函数，它接受一个由 Fn 约束的泛型参数 F ，并调用它
fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义一个满足 Fn 约束的闭包
fn function() {
    println!("我是函数!");
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move ||println!("这是一个：{}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("这是一个: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("这是一个: {}", text)
}

trait Iterator {
    // 被迭代的类型
    type Item;
    // any 接受 &mut self，意味着调用者可能被借用和修改，但不会被消耗
    fn any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool;

    // find 接受 &mut self , 这意味着调用者可能被借用和修改，但不会被消耗
    // FnMut 表示任何捕获的变量最多只能被修改，不能被消耗
    // &Self::Item 表示它通过引用将参数传递给闭包
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where P: FnMut(&Self::Item) -> bool;

}

// 是否为奇数
fn is_odd(n: i32) -> bool {
    n % 2 == 1
}

// 发散函数永不返回。它们使用 ! 标记，这是一个空类型。
fn foo() -> ! {
    panic!("这个函数永远不返回值。");
}