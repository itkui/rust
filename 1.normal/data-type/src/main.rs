/*
 * @FileName: 数据类型
 * @Description:
 * @Date: 2024-11-12 17:22:58
 */
fn main() {
    // let guess = "42".parse().expect("Not a number!"); // error: type must be known at this point 使用 parse 将 String 转换为数字时，必须增加类型注解
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess); // 42

    /**
     * 标量类型
     * 整型、浮点型、布尔类型和字符类型
     * */

    /**
     * 整型
     * 整数类型、进制数字字面值
     *
     * 长度      有符号   无符号    范围
     * 8-bit	i8	    u8       （-266）0 ~ 255
     * 16-bit	i16	    u16
     * 32-bit	i32	    u32
     * 64-bit	i64	    u64
     * 128-bit	i128	u128
     * arch	    isize	usize
     * */
    let mut n: u8 = 234;
    // n = 256; // error: literal out of range for `u8`，debug 环境会panic，prod 不会，会二进制补码
    println!("n: {}", n);

    /**
     * 浮点型
     * f32 和 f64（默认）
     * f32 是单精度浮点数，f64 是双精度浮点数
     */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Rust 中的整数除法使用向零截断，结果为 -1
    let truncatedF: f32 = -5.0 / 3.0;

    // remainder
    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("remainder: {}", remainder);
    println!("truncatedF: {}", truncatedF);

    /**
     * 布尔类型
     * bool
     */
    let t = true;

    let f: bool = false; 

    /**
     * 字符类型
     * char
     */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /** 
     * 复合类型
     * 元组、数组
    */
}
