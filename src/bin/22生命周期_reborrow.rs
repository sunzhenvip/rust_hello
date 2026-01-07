fn main() {}

struct S;

// Return short('b) reference
// 隐性约定 'a : 'b 关系  ‘a 活的比 'b 长
fn f1sr<'b, 'a>(rb: &'b &'a S) -> &'a S {
    // 反映的是 'b 短的那个引用
    // &T 是可copy的
    *rb // 如果返回 &'a S  可以替换 &'b S  因为 长生命周期可以替换短生命周期
}

fn f2sr<'b, 'a>(rb: &'b &'a mut S) -> &'a S {
    // &'a mut S 这部分不可被copy的
    *rb // 报错
        // 'c : 'b
        // 'a : 'b
        // 'c 转换 'a 有问题 因为 'c 和 'a 没有关系 没有生命周期关系
}

fn f2sr_1<'b: 'a, 'a>(rb: &'b &'a mut S) -> &'a S {
    *rb // 为什么编译通过了？
}

fn f3sr<'b, 'a>(rb: &'b mut &'a S) -> &'a S {
    *rb // &'c S(不可变引用 可被copy的) =>
}

fn f4sr<'b, 'a>(rb: &'b mut &'a mut S) -> &'a S {
    *rb // &'a mut S 转换为 &'c mut S => 'c : 'b
}
