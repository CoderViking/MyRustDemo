/// # 转换
/// 原生类型可以通过类型转换相互转换
/// Rust通过使用特质来处理自定义类型(如 struct 和 enum)之间的转换，通用转换使用 From 和 Into 特质。
/// 然而，对于更常见的情况，特别是与 string 相互转换时，还有一些更具体的特质。

pub fn run() {
    /**
     * From 和 Into
     * From 和 Into 特质本质上是相互关联的，这实际上是其实现的一部分。
     * 如果你能将类型 A 从类型 B 转换，那么我们也应该能够将类型 B 转换为类型 A。
     */

    /**
     * From
     * From 特质允许一个类型定义如何从另一个类型创建自身，从而提供了一种非常简单的机制来在多种类型之间进行转换。
     * 标准库中有许多这个特质的实现，用于原始类型和常见类型的转换。
     */

    let my_str = "hello";
    println!("my_str = {}", my_str);
    let my_string = String::from(my_str);
    println!("my_string = {}", my_string);

    // let num = Number::from(80);
    // println!("我的数字是: {:?}", num);

    /**
     * Into
     * Into 特质简单来说就是 From 特质的反向操作。它定义了如何将一个类型转换为另一个类型。
     * 调用 into() 通常需要我们指定结果类型，因为编译器大多数时候无法确定这一点。
     */

    let int = 5000;
    let num:Number = int.into();
    println!("我的数字是: {:?}", num);

    /*
     * From 和 Into 是可互换的
     * From 和 Into 被设计为互补的。我们不需要为两个特质都提供实现。
     * 如果你为你的类型实现了 From 特质，Into 会在必要时调用它。
     * 但请注意，反过来并不成立：为你的类型实现 Into 不会自动为它提供 From 的实现。
     */

    /**
     * TryFrom 和 TryInto
     * 与 From 和 Into 类似，TryFrom 和 TryInto 是用于类型转换的泛型特质。
     * 与 From 和 Into 不同，TryFrom 和 TryInto 特质用于可能失败的转换，因此返回 Result。
     */
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    /**
     * String 类型转换
     * 将任何类型转换为String 只需要为该类型实现 ToString 特质即可。
     * 但更友好的做法是实现 fmt::Display 特质，它不仅会自动提供 ToString，还允许打印该类型，就像在 println! 部分讨论的那样。
     */

    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "圆的半径为: {}", self.radius)
        }
    }

    let circle = Circle{ radius: 5.0 };
    println!("{}", circle);
    let circle_str = circle.to_string();
    println!("circle_str = {}", circle_str);
    let circle_str_append = format!("{} {}", "hello",circle_str);
    println!("circle_str_append = {}", circle_str_append);

    /**
     * 解析字符串
     * 将字符串转换为其他类型很有用，其中最常见的操作之一是将字符串转换为数字。
     * 管用方法是使用 parse 函数，可以通过类型判断或使用 ‘涡轮鱼’ 语法指定要解析的类型。
     * 只要为目标类型实现 FromStr 特质，就可以将字符串转换为指定类型。标准库中为许多类型实现了这个特质。
     * 要在自定义类型上获得这个功能，只需要为该类型实现 FromStr 特质
     */

    // 文本中只能是纯数字，
    // 例如：100_000， 这种文本则无法通过字符串直接转换成数字，需要先将下划线清除掉之后才可以转换;
    // 还有科学计数法(1e5)也不支持直接转换为数字
    let parsed: i32 = "50_000".replace("_", "").parse().unwrap();
    println!("parsed Number = {}", parsed);
    let turbo_parsed = "10678".parse::<i32>().unwrap();
    println!("turbo_parsed Number = {}", turbo_parsed);

    let sum = parsed + turbo_parsed;
    println!("sum = {}", sum);

    // 为自定义的类型实现 FromStr 特质
    impl FromStr for Circle {
        type Err = ParseFloatError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().parse() {
                Ok(num) => Ok(Circle{ radius: num }),
                Err(e) => Err(e),
            }
        }
    }

    let radius = "  500.887";
    let circle_parsed: Circle = radius.parse().unwrap();
    println!("circle = {}", circle_parsed);

}
// 为自定义的类型实现类似的转换

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
use std::convert::Into;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
// /// 实现 Into 时，不能已经实现了 From，因为新版本的Rust中实现From时已经实现了Into，因此再实现一次，会编译错误。
// /// 因此不推荐再单独实现 Into，一般只需要实现 Into 即可
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

// 为 EvenNumber 实现 TryFrom 函数
impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        // 能被2整除的数可以正常转换，除此之外的数都返回失败
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}