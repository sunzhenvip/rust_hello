// 生命周期 reborrow
// &T 实现了Copy &mut T 没有实现Copy
// 以下例子
// Rust 对 reborrow 还没有较好的文档，见 issue：https://github.com/rust-lang/reference/issues/788
// example 20-23 来自： https://cheats.rs/#memory-lifetimes
fn main() {
    let mut i = 32;
    let x = &mut i;

    change_i(x);
    println!("i = {}",x);

    *x = 44;
    println!("{}", *x);
}

fn change_i(mut_i32: &mut i32) { // x 重借用到了这里面 
    *mut_i32 = 43;
}
