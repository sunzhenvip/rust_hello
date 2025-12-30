use std::slice;

pub struct Bag<T> {
    items: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    // 1) iter：只读
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.items.iter()
    }

    // 2) iter_mut：可变
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.items.iter_mut()
    }
}

// 3) IntoIterator：消费 Bag -> 产出 T
impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// 4) IntoIterator：借用 &Bag -> 产出 &T
impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

// 5) IntoIterator：可变借用 &mut Bag -> 产出 &mut T
impl<'a, T> IntoIterator for &'a mut Bag<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

fn main() {
    let mut bag = Bag::new(vec![1, 2, 3]);

    // 只读遍历：&T
    for x in bag.iter() {
        println!("iter: {x}");
    }

    // 可变遍历：&mut T
    for x in bag.iter_mut() {
        *x += 10;
    }

    // for 语法（借用）：等价于 IntoIterator for &Bag
    for x in &bag {
        println!("&bag: {x}");
    }

    // 消费遍历：T（bag 被 move）
    for x in bag {
        println!("moved out: {x}");
    }
}
