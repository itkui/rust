/*
 * @FileName: 控制流
 * @Description:
 * @Date: 2024-11-15 14:33:04
 */

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("number was 5");
    } else {
        println!("condition was false");
    }

    // if number {
    //     // error: Rust 期望一个 bool 却得到了一个整数，需要显式地使用布尔值
    //     println!("number was true");
    // }

    let_if();

    loops();
}

/**
 * 在 let 语句中使用 if
 */
fn let_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // error:  if 的每个分支的可能的返回值都必须是相同类型

    println!("The value of number is: {}", number);
}

/**
 * 使用循环重复执行代码
 */

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            // 使用 break 关键字返回值 counter * 2。循环之后，我们通过分号结束赋值给 result 的语句
            // 为什么这里又需要分号了呢？
            // 因为if 最后一行不带分号是一个表达式，这里带分号，是因为是break语句，break语句可以携带一个返回值作为loop表达式的值。
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    loops_tag();
}

/**
 * 循环标签
 * 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环
 */
fn loops_tag() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    while_loop();
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for_loop();
}

fn for_loop() {
    const NAMES: [&str; 4] = ["Gandalf", "Frodo", "Sam", "Merry"];

    for element in NAMES {
        println!("{element}");
    }
    // 以下代码不会踏足到数字 4，仅从一个数字开始到另一个数字之前
    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
