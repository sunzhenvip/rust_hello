use std::marker::PhantomData;

struct Bar<'r> {
    _phantom: PhantomData<fn(&'r ())>, // 函数参数 生命周期是逆变的
}

fn foo<'short, 'long: 'short>(mut short_foo: Bar<'short>, mut long_foo: Bar<'long>) { // 看谁是子类
    // short_foo = long_foo; // 报错
    long_foo = short_foo; // 因为是 逆变 和 型变相反
}

fn main() {
    println!("Hello, world!");
}
