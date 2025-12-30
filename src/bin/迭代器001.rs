// 引用迭代器接口
use std::iter::IntoIterator;
use std::iter::Iterator;
// 一个“生成器式”的迭代器：从 start 到 end-1
struct CounterIter {
    cur: i32,
    end: i32,
}

impl Iterator for CounterIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            None
        } else {
            let x = self.cur;
            self.cur += 1;
            Some(x)
        }
    }
}

fn main() {
    // 这里 it 本身就是迭代器（Iterator）
    let it = CounterIter { cur: 0, end: 3 };

    for x in it {
        // ✅ OK：因为 it 实现了 IntoIterator（所有 Iterator 都自动实现 IntoIterator）
        println!("{x}");
    }

    // 但注意：你不能写 for x in CounterIter::new(...) 若没这个构造不重要
    // 核心点：你遍历的是“迭代器对象”，不是“容器对象”
}

fn max_num<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a > *b {
        a
    } else {
        b
    }
}

fn max_num1<'a, 'b>(a: &'a i32, b: &i32) -> &'a i32 {
    a
}
