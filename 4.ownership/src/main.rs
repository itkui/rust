fn main() {
    let x = 5;
    let y = x; // 内存拷贝，最终是x = 5,y = 5

    let xstr = String::from("hello");
    let ystr = xstr; // 所有权转移 xstr -> hello  => ystr -> hello。原因：如果是拷贝，面对较大堆内存拷贝时，性能会变的很低。如果是指针变更，两个变量在作用域结束时回收会回收两次，出现 二次释放（double free）错误
                     // 这里 hello 字符串的堆内存所有权就转移给了ystr，因此访问xstr时候会报错，xstr 不再有效
                     // println!("{}", xstr);  // error: borrow of moved value: `xstr`

    let clone_xstr = ystr.clone(); // xstr 已经无效了，hello 所有权被转移到了ystr，因此要使用ystr clone
    println!("{}", clone_xstr);

    main2();
}

fn main2() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ... ... 所以到这里不再有效
                        // println!("{s}"); // error: borrow of moved value: `s`

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处
