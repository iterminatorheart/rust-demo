fn main() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。
    println!("{} days", 31i64);

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数
    println!("this is position {1}, and this is position {0}, that, this is position {}", 1, 2);

    // 可以使用命名参数。
    println!("hi, my name is {wang}, and my computer is {apple}", wang="王", apple="苹果");

    // 可以在 `:` 后面指定特殊的格式。
    println!("the binary format of {} is {0:b}", 7);

    // 还可以指定在数字左边补0
    println!("the binary format of {} is {0:>0width$b}", 7, width = 8);

    // 也可以指定在数字左边补空格
    println!("the binary format of {} is {0:>width$b}", 7, width = 8);

    // exercise 通过使用 println! 宏，通过控制显示的小数位数来打印：Pi is roughly 3.142
    print_pi();
}

fn print_pi() {
    let pi = 3.141592;
    // println!("Pi is roughly {0:.3}", pi);
    println!("Pi is roughly {0:.3}", pi);
}