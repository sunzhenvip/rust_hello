fn main() {}

struct S;

// Return short('b) reference
// 隐性约定 'a : 'b 关系  ‘a 活的比 'b 长
fn f1lm<'b, 'a>(rb: &'b &'a S) -> &'a mut S {
    *rb // 报错
}

fn f2lm<'b, 'a>(rb: &'b &'a mut S) -> &'a mut S {
    *rb // 报错
}

fn f3lm<'b, 'a>(rb: &'b mut &'a S) -> &'a mut S {
    *rb // 报错
}

fn f4lm<'b, 'a>(rb: &'b mut &'a mut S) -> &'a mut S {
    *rb // 报错
}

fn f4lm_1<'b: 'a, 'a>(rb: &'b mut &'a mut S) -> &'a S {
    *rb // 通过
}
