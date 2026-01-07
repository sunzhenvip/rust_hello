// RFC 1951 : https://rust-lang.github.io/rfcs/1951-expand-impl-trait.html
// RFC 2394 : https://rust-lang.github.io/rfcs/2394-async_await.html
// https://github.com/chen4903/Index-Rust/blob/main/01.基础入门/10.生命周期/04.Impl Trait生命周期.md
// impl trait 生命周期, impl trait lifetime
trait Foo {}

impl Foo for &'_ str {}

fn f1<T: Foo>(t: T) -> Box<impl Foo> {
    // impl Foo 会扑获 T
    Box::new(t) // 编译通过
}

fn f2<T: Foo>(t: T) -> Box<dyn Foo> {
    // dyn Foo 不会扑获 T
    Box::new(t) // 编译失败
}

fn f2_1<T: Foo + 'static>(t: T) -> Box<dyn Foo> {
    Box::new(t) // 编译通过
}

fn f2_2<'a, T: Foo + 'a>(t: T) -> Box<dyn Foo + 'a> {
    Box::new(t) // 编译通过
}

fn f3(s: &'static str) -> Box<impl Foo> {
    Box::new(s)
}

fn f3_1<'a>(s: &'a str) -> Box<impl Foo> {
    Box::new(s)
}

fn f3_2<'a>(s: &'a str) -> Box<impl Foo + 'a> {
    Box::new(s)
}

fn f3_3(s: &str) -> Box<impl Foo + '_> {
    Box::new(s)
}

fn f3_4(s: &str) -> Box<impl Foo> {
    Box::new(s)
}

fn f4(s: &str) -> Box<dyn Foo> {
    Box::new(s)
}

fn f4_1(s: &str) -> Box<dyn Foo + 'static> {
    Box::new(s)
}

fn f4_2(s: &str) -> Box<dyn Foo + '_> {
    Box::new(s)
}

fn main() {}
