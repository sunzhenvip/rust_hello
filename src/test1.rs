





fn main() {

    let a = Box::new(42);  // Box<i32>，堆分配的整数
    let b = a;             // 所有权转移，a 不再可用
    // println!("a = {}", a);  // 编译错误！a 的所有权已经转移
    println!("b = {}", b);  // 正确


    //  不可变性
    /*    let mut nice_count = 100;

    nice_count = 1;
    println!("Hello, world!{}", nice_count);*/

    let x = 5; // mut

    {
        let x = 10;
        println!("内部 x = {}", x);
    }
    println!("外部 x = {}", x);

    let mut x = "hello";

    x = "xxx";
    println!("外部 x = {}", x);
}
