// fn max_of_refs<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
//     if *a > *b {
//         a
//     } else {
//         b
//     }
// }
//  表示 a的生命周期 至少要和 a b 变量(生命周期 )遍历一样长  返回值 这个引用 不能长于 a 和 b
// 也可以 说 返回值的生命周期 至少和 a b(引用的变量) 一样长

fn complex_function(a: &i32) -> &i32 {
    let b = 2;
    max_of_refs(a, &b)
}

fn max_of_refs<'a, 'b>(a: &'a i32, b: &'b i32) -> &'a i32 { //
    // let b = 2;
    // max_of_refs(a, &b)
    a
}

fn main() {
    let a = 1;
    let my_num = complex_function(&a);
    // println!("{my_num}");
}
