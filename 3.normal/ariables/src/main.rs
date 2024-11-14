/*
 * @FileName: 变量和可变性
 * @Description:
 * @Date: 2024-11-12 16:59:59
 */
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    // x = 6; // error： cannot assign twice to immutable variable
    // println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /**
     * 常量
     * 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const THREE_HOURS_IN_SECONDS_COMPUT: u32 = 10800;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of THREE_HOURS_IN_SECONDS_COMPUT is: {THREE_HOURS_IN_SECONDS_COMPUT}");

    /**
     * 隐藏
     */
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }
    println!("The value of y is: {y}"); // 6

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}"); // 3

    let mut spaces = "   ";
    // spaces = spaces.len(); // error：expected `&str`, found `usize`
    println!("The value of spaces is: {spaces}");
}
