// 生命周期 reborrow
// &T 实现了Copy &mut T 没有实现Copy
// 以下例子
// Rust 对 reborrow 还没有较好的文档，见 issue：https://github.com/rust-lang/reference/issues/788
// example 20-23 来自： https://cheats.rs/#memory-lifetimes
fn main() {
    let mut i = 32;
    let x = &mut i;

    // let y: &mut i32 = x; // 等价后面写法
    let y = &mut *x; // y 可变引用 reborrow 重引用
    // let y: &mut i32 = &mut *x; // 等价写法

    *y = 43;
    println!("{}", *y);

    *x = 44;
    println!("{}", *x);
    
}
