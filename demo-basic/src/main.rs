mod l01_print_demo;
mod l02_primitive_types_demo;
mod l03_custom_type_demo;
mod l04_var_bind_demo;
mod l05_rust_type_demo;
mod l06_onversion_demo;

use common;
fn main() {
    println!("Hello, world!");
    // 调用外部carte中的函数
    common::hello();
    // 调用外部工具函数库中的函数
    common::utils::get_date();
    // 调用工具函数库中单独公开的函数
    common::say_hello();
    // ============ 格式化输出 demo 代码 ================
    // l01_print_demo::run();
    // ============ 原生类型 demo 代码 ================
    // l02_primitive_types_demo::run();
    // ============ 自定义类型 demo 代码 ================
    // l03_custom_type_demo::run();
    // ============ 变量绑定 demo 代码 ================
    // l04_var_bind_demo::run();
    // ============ 类型转换 demo 代码 ================
    // l05_rust_type_demo::run();
    // ============ 类型转换 demo 代码 ================
    l06_onversion_demo::run();
}


