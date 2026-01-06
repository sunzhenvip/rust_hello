fn main() {}

struct S;

// Return short('b) reference
// 隐性约定 'a : 'b 关系  ‘a 活的比 'b 长
fn f1sr<'b, 'a>(rb: &'b &'a S) -> &'b S {
    // 反映的是 'b 短的那个引用
    // &T 是可copy的
    *rb // 如果返回 &'a S  可以替换 &'b S  因为 长生命周期可以替换短生命周期
}

fn f2sr<'b, 'a>(rb: &'b &'a mut S) -> &'b S {
    // &'a mut S 这部分不可被copy的
    *rb // 得到的是 另外一个生命周期 &'c mut S => 'a : 'b ，然后 'c : 'b 这样的关系
        // 那么 长的返回一个短的 那就没问题 即是 b
        // 最后 可变 &'a mut S 转换成 不可变 &'b S 没问题
}

fn f3sr<'b, 'a>(rb: &'b mut &'a S) -> &'b S {
    *rb // 得到的是 &'c S  因为是可copy的
}

fn f4sr<'b, 'a>(rb: &'b mut &'a mut S) -> &'b S {
    *rb // &'a mut S 转换为 &'c mut S => 'c : 'b
}
