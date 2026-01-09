use std::marker::PhantomData;

struct Foo<'r> {
    _phantom: PhantomData<&'r ()>, //针对不可变引用 &r  是斜变
}

// 测试协变的情况
fn foo<'short, 'long: 'short>(mut short_foo: Foo<'short>, mut long_foo: Foo<'long>) {
    short_foo = long_foo; //  因为 lang是 short 子类 所以 lang 可以赋值给 父类 long 的 含义是子类
    // long_foo = short_foo; // 报错
}

fn main() {
    println!("Hello, world!");
}

// 子类型 可以安全替换 父类型
// 子类 可以 赋值 给 父类 使用

// long 是 short 的 子类
// long 的 生命周期比 short 长

// 子类对象 可以赋值 给父类类型 的引用
