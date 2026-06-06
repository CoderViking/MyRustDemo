/// # 自定义类型
/// rust中自定义数据类型主要由以下两个关键字来创建的：
/// - struct: 定义结构体
/// - enum: 定义枚举
/// 常量也可以通过 const 和 static 关键词来创建


/// 结构体
/// 使用 struct 关键词可以创建三种类型的结构体
/// - 元组结构体：本质上是具名结构体
/// - 单元结构体：没有字段，在泛型中很有用
/// - 经典的 C 语言风格结构体

// 这个属性用于隐藏未使用代码的警告
#[allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// 定义单元结构体
#[derive(Debug)]
struct Unit;

// 定义元组结构体
#[derive(Debug)]
struct Pair(i32, f32);

// 定义带有两个字段结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体也可作为另一个结构体的字段类型
struct Rectangle {
    // 可以通过指定左上角和右下角的位置来定义一个矩形
    top_left: Point,
    bottom_right: Point,
}
// 根据矩形两个顶点的坐标计算矩形的面积
fn rectangle_area_count(rectangle: Rectangle) -> f32 {
    // 采用嵌套解构方式
    let Rectangle{top_left: Point{x: point_one_x, y: point_one_y}, bottom_right: Point{x: point_two_x, y: point_two_y}} = rectangle;
    ((point_one_y - point_two_y) * (point_one_x - point_two_x)).abs()
}
pub fn run () {
    // 使用字段初始化简写语法创建结构体
    let name = String::from("张三");
    let age = 25;
    let zhang_san = Person{name, age};

    // 打印结构体的调试信息
    println!("Hello, {:?}", zhang_san);

    // 实例化一个 Point
    let point_one = Point {x:5.2, y: 0.4};
    let point_two = Point {x: 10.3, y: 10.2};

    println!("访问点坐标为：x={}, y={}", point_one.x, point_one.y);

    // 使用结构体更新语法创建新的点
    // 复用之前创建的点
    let bottom_right = Point { ..point_two };
    // 也可以对第一个参数进行自定义指定，第二个参数复用另一个结构体，反之则不行
    let new_point_one = Point{x:5.8, ..point_two};
    // let new_point_one = Point{..point_two, y:0.8}; // 编译错误

    println!("第二个访问点坐标为： x={}, y={}", bottom_right.x, bottom_right.y);
    println!("第三个访问点坐标为： x={}, y={}", new_point_one.x, new_point_one.y);

    // 使用let解构结构体
    let Point{x:left_point, y: right_point} = point_one;

    let rectangle_one = Rectangle{
        top_left: Point{x: left_point, y: right_point},
        bottom_right: bottom_right,
    };

    println!("矩形的坐标为：{}, {}, {}, {}", rectangle_one.top_left.x, rectangle_one.top_left.y,
                rectangle_one.bottom_right.x, rectangle_one.bottom_right.y);

    // 计算矩形的面积，保留两位小数
    println!("矩形的面积为：{:.2}", rectangle_area_count(rectangle_one));

    let _unit = Unit;

    println!("打印单元结构体：{:?}", _unit);

    let _pair = Pair(100, 0.000_001);

    println!("打印元组结构体：{:?}", _pair);
    println!("元组结构体包含：{}， {}", _pair.0, _pair.1);

    // 解构元组结构体
    let Pair(integer, decimal) = _pair;
    println!("元组结构体包含：{}， {}",integer, decimal);

    let new_rectangle = NewRectangle{height: 30, width: 50};
    println!("矩形面积计算：矩形:{:?}, 面积为: {}",new_rectangle,
             area_count( new_rectangle.height, new_rectangle.width));



    let loading = WebEvent::PageLoading;
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click {x: 3000, y: 800};
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("我的文本".to_owned());
    let unload = WebEvent::PageUnload;

    // 问题：问什么使用下面的函数传入枚举值时，一个枚举值只允许使用一次，不可重复使用？
    inspect(loading);
    inspect(load);
    inspect(click);
    inspect(pressed);
    inspect(pasted);
    inspect(unload);

    // 通过枚举的别名引用枚举变体，
    let x = Operations::Add; // 将枚举值变体赋值给一个变量，后续可以使用变量代替枚举值变体
    let y = Operations::Subtract;

    let add_result = Operations::Add.run(300, 500);
    let add_result1 = x.run(300, 500);
    let add_result2 = Operations::Subtract.run(300, 500);
    let add_result3 = y.run(300, 500);
    println!("使用枚举别名调用加法计算结果：300+500={}", add_result);
    println!("使用枚举别名调用加法计算结果：300+500={}", add_result1);
    println!("使用枚举别名调用加法计算结果：300-500={}", add_result2);
    println!("使用枚举别名调用加法计算结果：300-500={}", add_result3);


    // 显式 'use' 每个名称，使他们不需要手动作用限定域就可以使用
    use Stage::{ Beginning, Advanced};
    // 把 Role 下所有成员导入当前作用域
    use Role::*;

    // 等同于 'Stage::Beginning'
    let stage = Beginning;
    // 等同于 'Role::Student'
    let role = Student;

    match stage{
        // 注意由于上面的显式 'use'， 这里不需要作用域限定
        Beginning => println!("初学者正在开始他们的学习之道"),
        Advanced => println!("高级学习者正在掌握他们的科目..."),
    }

    match role {
        // 由于上面已显示 'use' ，因此这也不需要作用域限定
        Student => println!("学生正在获取知识"),
        Teacher => println!("教师正在传播知识"),
    }

    // enum 可以转换为整数
    println!("zero的值为：{}", Number::Zero as i32);
    println!("one的值为：{}", Number::One as i32);
    println!("two的值为：{}", Number::Two as i32);

    println!("玫瑰的颜色是：#{:06x}", Color::Red as u32);
    println!("紫罗兰的颜色是：#{:06x}", Color::Blue as u32);

    // 调用链表测试函数
    list_test();

    // 调用常量测试函数
    test_const();

}
#[derive(Debug)]
struct NewRectangle{
    height: u32,
    width: u32,
}

// 计算矩形面积
fn area_count(height:u32, width:u32) -> u32 {
    height * width
}

/// # 枚举
/// enum 关键字允许创建一个可能是几种不同变体之一的类型，任何作为struct有效的变体在enum中也是有效的。

// 创建一个 'enum' 来分类网页事件，注意名称和类型信息如何共同指定变体
// 每个变体都是不同且独立的
enum WebEvent {
    PageLoading,
    PageLoad,
    PageUnload,
    Paste(String),
    KeyPress(char),
    Click { x: i64, y: i64 },
}

// 一个接受 WebEvent 枚举作为参数，且不返回任何值的函数
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoading => println!("page loading"),
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        // 从 enum 变体内部解构 c
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        // 将 Click 解构为 x 和 y
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}.",x,y);
        },
        WebEvent::Paste(s) => println!("pasted '{}'.", s),
    }
}

/// # 类型别名
/// 使用类型别名可以通过别名引用每个枚举变体，
// 当枚举名称过长或过于泛化时，可以使用 'type' 关键字为枚举创建一个别名

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// 为枚举创建别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// 别名通常是在使用 self 别名的 impl 块中
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y:i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

/// # 可以使用 use 声明来避免输入完整的模块路径来访问名称
enum Stage {
    Beginning,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

/// # C 风格用法
/// enum 也可以像C语言那样使用

// 带隐式判别值的枚举 (从0开始)
enum Number {
    Zero,
    One,
    Two,
}
// 带显式判别值的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x0000ff,
    Blue = 0x0000ff00,
}

/// # 测试实例：链表
/// 使用 enum是实现链表的常用方式

use List::*;

enum List {
    // Cons: 包含一个元素和指向下一个节点指针的元组结构体
    Cons(u32, Box<List>),
    // Nil: 表示链表末尾的节点
    Nil,
}

// 可以为枚举实现方法
impl List {
    // 创建空链表
    fn new() -> List {
        // Nil 的类型是 List
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        // 需要的self进行匹配，因为方法的行为取决于self的变体
        match self {
            // 递归遍历每一个元素，每次长度 + 1
            Cons(_, tail) => 1 + tail.len(),
            // 基本情况，空链长度为0
            Nil => 0,
        }
    }
    // 返回链表的字符串表示(堆分配)
    fn stringify(&self) -> String {
        match self {
            // format! 类似于 println! 但返回的是堆分配的字符串，而不是打印到控制台
            Cons(head, tail ) => format!("{}, {}", head, tail.stringify()),
            Nil => "Nil".to_string(),
        }
    }

}

fn list_test() {

    // 创建一个空链表
    let mut list = List::new();

    // 在链表头部添加一些元素
    list = list.prepend(1);
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印链表的最终长度
    println!("链表的长度为: {}", list.len());
    println!("{}", list.stringify());
}

/// # 常量
/// rust中有两种常量类型，可在任何作用域(包括全局作用域)中声明，两者都需要显示类型标注：
/// - const: 不可变值(常见用法)
/// - static: 具有 static 可能可变值。静态生命周期会被自然推断，无需明确指定。
/// 访问或修改可变静态变量是不安全的
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn test_const() {
    let n = 18;

    println!("这是{}", LANGUAGE);
    println!("阈值为: {}", THRESHOLD);
    println!("{} 是 {}", n, if is_big(n) { "大的" } else { "小的" });

    // LANGUAGE = "Cargo"; // 编译错误：无法继续赋值给不可变变量

    // THRESHOLD = 12; // 编译错误：左侧操作数无效

}
