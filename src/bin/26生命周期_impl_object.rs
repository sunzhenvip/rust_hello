// RFC 1951 : https://rust-lang.github.io/rfcs/1951-expand-impl-trait.html
// RFC 2394 : https://rust-lang.github.io/rfcs/2394-async_await.html
// https://github.com/chen4903/Index-Rust/blob/main/01.基础入门/10.生命周期/04.Impl Trait生命周期.md
// impl trait 生命周期, impl trait lifetime
use std::future::Future;
fn main() {
    let future; // (1)
    {
        let s = String::from("any");
        future = f1(&s); // 可以编译通过为什么 s 在 当前执行完不就回收了吗
                         // future = f2(&s); // 编译失败为什么？
    }
    let _another_future = future;
}

fn f1(s: &str) -> impl Future<Output = ()> {
    // impl Future<Output = ()> 认为和 s 参数生命周期没关系
    async move {
        // println!("{}", s); // 编译失败
        ()
    }
}

fn f1_1(s: &str) -> impl Future<Output = ()> + '_ {
    async move {
        println!("{}", s); // 编译通过
        ()
    }
}

async fn f2(_s: &str) -> () {
    // async 会扑获入参的生命周期
    ()
}

async fn f2_1<'a, 'b>(s1: &'a str, s2: &'b str) -> () {
    // async 会扑获入参的生命周期 也会给到返回值
    ()
}

// 手工写  async Future
fn f2_3<'a, 'b>(s1: &'a str, s2: &'b str) -> impl Future<Output = ()> + 'a {
    // 可以指定返回值影响到了那几个生命周期
    println!("{}", s2);
    async move {
        println!("{}", s1);
        ()
    }
}
