// 参考资料
// RFC 387 : https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html
// RFC 1598 : https://rust-lang.github.io/rfcs/1598-generic_associated_types.html
// 加强理解 GAT 的一篇文章： https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats

fn f<'a>() {

}

fn g<'a: 'a>() {

}

fn main() {
    let ff = f::<'static> as fn(); // 报错
    let gg = g::<'static> as fn(); // 通过
}
