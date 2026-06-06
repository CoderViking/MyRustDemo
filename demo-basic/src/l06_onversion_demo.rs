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

// /// 实现 Into 时，不能已经实现了 From，因为新版本的Rust中实现From时已经实现了Into，因此再实现一次，会编译错误。
// /// 因此不推荐再单独实现 Into，一般只需要实现 Into 即可
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }