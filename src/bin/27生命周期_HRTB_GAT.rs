// 参考资料
// RFC 387 : https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html
// RFC 1598 : https://rust-lang.github.io/rfcs/1598-generic_associated_types.html
// 加强理解 GAT 的一篇文章： https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
fn main() {}

/*fn call_on_ref_zero<'a, F>(f: F)
// 一般这里'a 要比 这个函数体 要长
where
    F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}*/

fn call_on_ref_zero_1<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn call_on_ref_zero_2<F>(f: F)
where
    F: Fn(&i32),
{
    let zero = 0;
    f(&zero);
}
