use std::marker::PhantomData;

struct Bar<'r> {
    _phantom: PhantomData<fn(&'r ())>, // 函数参数 生命周期是逆变的
}

fn foo<'short, 'long: 'short>(mut short_foo: Bar<'short>, mut long_foo: Bar<'long>) { // 看谁是子类
    // short_foo = long_foo; //  因为 lang是 short 子类 所以 lang 可以赋值给子类
    long_foo = short_foo; // 报错
}

fn main() {
    println!("Hello, world!");
}
