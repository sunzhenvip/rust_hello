use std::fmt::{Debug, Display};
use std::task::Waker;
use std::borrow::Borrow;
fn get<'a>() -> &'a i32 {
    &200
}

fn test1(ss: &Vec<i32>) {
    println!("s={:?}", ss);
}




fn main() {


    let n1 = 200;

    // n1.to_string()
    let a = &n1 as *const i32;

    println!("a={:?}", a);

    let name = String::from("111");

    let data1 = vec![1, 2, 3];
    test1(&data1);

    println!("data1={:?}", data1);

    let res = &name;



    println!("{}", res);
    println!("{}", name);

    // let b = 10;
    //
    // let c: &i32 = &b;
    //
    // println!("b={}, c={}", b, c);

    /*let a = &100;
    let b = &100;

    let cde = a + b;
    println!("cde = {}",cde)*/
    // let ccc = get_num(&100, &200);
    // println!("ccc={}, ccc={}", ccc, ccc);

    {
        let s: &'static str = "hello world";
        println!("{}", s);
    }

    #[derive(Debug)]
    struct Point {
        x: (u32, u32),
        y: u32,
    }

    let p = Point { x: (1, 2), y: 3 };

    let Point { x, y } = p;
    println!("x = {:?} y = {:?}", x, y);

    let num = 10u8;

    match num {
        hello => println!("hello {}", hello),
        1 => println!("one"),
        2 => println!("two"),
    }
    // 数据解构


    let a = Actor {
        name: "周星驰".to_string(),
        movie: "少林足球".to_string()
    };
    println!("{}", a.to_string());
}

// 我在学生命周期

fn get_num<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}

struct Car<'a> {
    brand: &'a str,
}

impl<'a> Car<'a> {
    fn run<'c>(&'a self, num: &'c i32) {
        println!("{} -> {}", self.brand, num);
    }
}

struct Actor {
    name : String,
    movie:String,
}


impl Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {}", self.name, self.movie))
    }
}



