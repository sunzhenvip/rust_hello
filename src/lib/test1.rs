pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(err1: std::io::Error) -> Self {
        Error::IO(err1.kind())
    }
}


pub fn lib_test() {

    let aa = Box::new(8);

    //
    println!("lib.test1.rs->lib_test");
}




mod lib;
pub mod mod1;

use std::alloc::System;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use std::{mem, string};
use std::thread::sleep;

struct Pair(i32, f64);

#[derive(Debug)] // 派生属性
struct Person {
    name: String,
    age: u32,
}

enum IPAddr {
    IPv4(u8, u8, u8, u8),
    /* IPv6(
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    ),*/
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let four = Rc::new(List::Cons(4, Rc::new(List::Nil)));
    // Rc::clone(&four);
    // four.clone();
    // let zero_one =
    let mut v: Vec<i32> = Vec::new();

    for i in 0..10 {
        v.push(i);
    }

    let s: &'static str = "hello";

    println!("xxxxx = {:?}", v[8]);

    let mut tar1: HashMap<&str, u32> = HashMap::new();
    tar1.insert("tar1", 1);
    tar1.insert("tar2", 2);
    tar1.insert("tar3", 3);

    let mut aa = tar1.get("tar3");
    println!("{:?}", aa);
    // 宏
    // let mut v1 : Vec<i32> = vec![1, 2, 3];

    // v1.push(1);

    // 第三方库
    // 系统时间
    let now = SystemTime::now();

    let time = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("xxxxxxxxx {:?}", time); // 打印时间
    // 拥有自己的数据 可以进行修改
    let aaa = String::from("xx");

    lib::test1::lib_test();
    println!("{}", lib::MESSAGE);
    mod1::mod1_test();

    sleep(Duration::from_secs(4));

    println!("ela {:?}", now.elapsed().unwrap());
    // mut 可变变量
    let mut msg: String = String::new();
    let a: i32 = 10;
    let b: char = 'A';

    let my_tuple: (i32, char) = (a, b);

    let (c, d) = my_tuple;

    println!("c = {:?}, d = {:?}", c, d);

    println!("mytuple.0 = {:?}", my_tuple.0);

    // 元组结构
    // 标准C结构
    // 单元结构（无字段）

    // 元祖结构

    let pair1 = Pair(10, 4.2);

    println!("aaa = {:?} bbb = {:?}", pair1.0, pair1.1);

    let jack = Person {
        name: String::from("jack"),
        age: 6,
    };

    print!("name = {},age = {}\n", jack.name, jack.age);
    print!("{:?}", jack);

    let i: isize = 1;
    let j: isize = foo(i);
    println!("\njjjjjj = {}", j);

    let localhost: IPAddr = IPAddr::IPv4(127, 0, 0, 0);
    match localhost {
        IPAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d)
        } /*_ => {} // 如果不是ipv4 走到这个分支 目前什么都不做*/
    }

    unsafe {
        let a1 = [0u8, 1u8, 0u8, 0u8];
        let b1: u32 = mem::transmute(a1);
        print!("b1 = {}", b1);
    }

    let mut sum = 0;
    let mut n = 1;

    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break;
        }
    }
    println!("1 + 2 + ... + 100 = {}", sum);
    /*  for pat in expr {
        unimplemented!();
    }
    let name: &str = "Alice";  // 类型是 &str*/
}

fn foo(mut i: isize) -> isize {
    i += 1;
    i
}
