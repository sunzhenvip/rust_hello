use std::marker::PhantomData;

struct Foo<'r> {
    _phantom: PhantomData<&'r ()>,
}

fn foo<'short, 'long: 'short>(mut short_foo: &mut Foo<'short>, mut long_foo: &mut Foo<'long>) {
    // short 和 long 这种情况下是不是没啥关系了？ 因为生命周期不能被协变或者逆变了？
    // short_foo = long_foo; // 会报错
    // long_foo = short_foo; // 会报错
    // 会出现多次借用的问题
}

fn main() {
    println!("Hello, world!");
}
