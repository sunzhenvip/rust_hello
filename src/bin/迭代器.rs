// 引用迭代器接口
use std::iter::IntoIterator;
use std::iter::Iterator;

#[derive(Debug)]
struct Range {
    counter: i32,
}

impl Range {
    fn new() -> Self {
        Range { counter: 0 }
    }
    fn test(&self) {
        println!("test");
    }
    fn demo(&self) {
        println!("test");
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 2 {
            return None;
        }
        self.counter += 1;
        Some(self.counter)
    }
}

fn get<'a>() -> &'a i32 {
    &20
}

fn main() {
    let mut r = Range::new();

    // println!("{:?}", r);
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());
    // println!("{:?}", r.next());

    for i in r {
        println!("{}", i);
    }

    let sa  = get();
    println!("{}", sa);
}
