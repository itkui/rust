/*
 * @FileName:
 * @Description:
 * @Date: 2024-11-14 15:35:20
 */
fn main() {
    println!("Hello, world!");
    // other_fn(21, "h"); // error: 字符字面量用'单引号，字符串字面量"双引号
    other_fn(21, 'h');
    main2();
}

fn other_fn(u: i32, t: char) {
    println!("other_fn u: {}, t: {}", u, t);
}

/**
 * 表达式与返回值
 *
 * 函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式
 *  */

fn main2() {
    let y = {
        let x = 3;
        x + 1 // 表达式没有分号，如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The five value of x is: {x}");

    let x2 = plus_one(x);

    println!("The plus_one value of x2 is: {x2}");

    let x3 = test_fn();
    println!("The test_fn value of x3 is: {x3}");
}

/**
 * 具有返回值的函数
 */

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 无分号，表达式
          // x + 1; // 有分号，语句，报错
}

fn test_fn() -> char {
    let x = 'a';
    x
}
