fn main() {
    let mut my_vec = vec![];
    let val1 = 1;
    let val2 = 2;

    insert_value(&mut my_vec, &val1);
    // insert_value(&mut my_vec, &val2); // 这一行为什么编译错误 提示  cannot borrow `my_vec` as mutable more than once at a time
    // 违反了可变引用多次借用的问题
    // println!("{my_vec:?}");
}

fn insert_value<'a>(my_vec: &'a mut Vec<&'a i32>, value: &'a i32) {
    my_vec.push(value);
}

// fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
//     my_vec.push(value)
// }

// fn insert_value<'r, 'val>(my_vec: &'r mut Vec<&'val i32>, value: &'val i32) {
//     my_vec.push(value);
// }

// fn insert_value1(my_vec: &mut Vec<&i32>, value: &i32) {
//     my_vec.push(value);
// }
