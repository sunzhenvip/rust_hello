struct Bag<T> {
    items: Vec<T>,
}

impl<T> Bag<T> {
    fn new(items: Vec<T>) -> Self { Self { items } }
}

// 1) for x in bag （move，产出 T）
impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// 2) for x in &bag （borrow，产出 &T）
impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

// 3) for x in &mut bag （mut borrow，产出 &mut T）
impl<'a, T> IntoIterator for &'a mut Bag<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;


    fn into_iter(self) -> Self::IntoIter {
    // 调用内部 items 的 iter_mut() 方法，返回一个可变迭代器
    // 这个迭代器允许遍历并修改集合中的元素
        self.items.iter_mut()
    }
}

//
fn main() {

    let  v = vec![1, 2, 3];
    // for x in v {
    //     println!("val={x}");
    // }
    for x in v.into_iter() {
        println!("val={x}");
    }
    // for (i, x) in v.iter().enumerate() {
    //     println!("idx={i}, val={x}");
    // }


    let bag = Bag::new(vec![1, 2, 3]);


    // 只读遍历：不消耗 bag
    for x in &bag {                 // x: &i32
        println!("&bag: {x}");
    }

    // 可变遍历：不消耗 bag，但能改内部元素
    let mut bag2 = Bag::new(vec![10, 20]);
    for x in &mut bag2 {            // x: &mut i32
        *x += 1;
    }
    for x in &bag2 {
        println!("bag2 after: {x}"); // 11, 21
    }

    // 消费遍历：bag3 被 move 掉，之后不能再用
    let bag3 = Bag::new(vec![7, 8, 9]);
    for x in bag3 {                 // x: i32
        println!("bag3 moved: {x}");
    }
    // println!("{:?}", bag3.items); // ❌ 编译不过：bag3 已被 move
}
