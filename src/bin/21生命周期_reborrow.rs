fn main() {}

struct S;

fn f1sm<'b, 'a>(rb: &'b &'a S) -> &'b mut S {
    *rb // 编译失败
}

fn f2sm<'b, 'a>(rb: &'b &'a mut S) -> &'b mut S {
    *rb // 编译失败
}

fn f3sr<'b, 'a>(rb: &'b mut &'a S) -> &'b mut S {
    *rb // 编译失败
}

fn f4sr<'b, 'a>(rb: &'b mut &'a mut S) -> &'b mut S {
    *rb // 编译通过
}
