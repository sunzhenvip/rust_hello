#![allow(unused_variables)] // 忽略未使用变量警告:ml-citation{ref="2,5" data="citationList"}
#![allow(dead_code)] // 忽略未使用代码警告:ml-citation{ref="2" data="citationList"}
#![allow(unused_assignments)] // 忽略未使用的赋值操作:ml-citation{ref="2" data="citationList"}


use std::fmt::Debug;
use std::fs;
use std::rc::Rc;
use std::sync::Arc;

fn foo<T: Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

#[allow(unused_imports)]
fn main() {
    /*let arr1 = [true, true, true];
    let arr2 = [6, 10, 300, 600];
    foo(arr1);
    foo(arr2);
    let arr3 = [6, 10, 300, 600];
    println!("{:?}", arr3);

    const fn double(n: usize) -> usize {
        n * 2
    }

    fn print<const N: usize>() {
        println!("N = {}", N);
    }

    print::<{ double(3) }>(); // N = 6

    let v1: Option<i32> = Some(200);
    let v2: Option<i32> = Some(200);
    let _v3 = Some(200);
    let v4: Option<i32> = None;
    println!("{}", v1 == v2);
    println!("{:?}", v4);

    // let v1 = Option::None;
    let v1 = Some(Age { age: 1 });
    let xx = v1.unwrap();
    println!("xx {:?}", xx);
    println!("v1 {:?}", v1);

    let r = fs::create_dir("./some");
    // let res = r.unwrap();
    let res = r.is_err();
    println!("{:?}", res); //成功 ()

    let r = status_code();
    // let res= r.unwrap_or(100); // 返回一个OK的默认值

    // let res= r.unwrap_or_else(| err | {println!("页面不存在了 {}", err); 404});
    let res = r.map(|nf| if nf > 500 { true } else { false });

    println!("22");
    println!("111 {:?}", res);

    let c = |code| {
        println!("闭包 操作成功 {}", code);
        Ok(200)
    };
    let res = request_successed().and_then(request_failed).and_then(c);

    println!("-----------------------------");

    let r = Rc::new(200);
    println!("{:?}", Rc::strong_count(&r));
    println!("{:?}", r);
    let r1 = Rc::clone(&r);
    println!("{:?}", Rc::strong_count(&r));
    println!("{:?}", r1);
    let r2 = Rc::clone(&r);
    println!("{:?}", Rc::strong_count(&r));
    println!("{:?}", r2);
    {
        let r3 = Rc::clone(&r);
        println!("{:?}", Rc::strong_count(&r));
        println!("{:?}", r3);
    }
    println!("{:?}", Rc::strong_count(&r));

    let mut r8 = Rc::new(2010);
    println!("{:?}", *r8);*/

    // *r = 200;
    // println!("{:?}", r);

    println!("---------------------------------------");



    let mut r9 = Rc::new(200);
    println!("{:?}", r9);
    *Rc::get_mut(&mut r9).unwrap() = 3000000;


    let r10 = Rc::clone(&r9);

    // *Rc::get_mut(&mut r9).unwrap() = 300;

    println!("{:?}", r9);
    println!("{:?}", r10);
}

#[derive(Debug, Copy, Clone)]
struct Age {
    age: i32,
}

/*fn foo1<T: Debug, N>(arr: [T; N]) {
    println!("{:?}", arr);
}
*/

/*struct A<T>
where
    T: Iterator, // 可以用 A<T: Iterator> 来替代
    T::Item: Copy,
    String: PartialEq<T>,
    // i32: Default,           // 允许，但没什么用
    // i32: Iterator,          // 错误: 此 trait约束不适合，i32 没有实现 Iterator
    // [T]: Copy,              // 错误: 此 trait约束不适合，切片上不能有此 trait约束
{
    f: T,
}
*/
#[derive(Debug)]
struct Phone;

fn buy_phone(money: i32) -> Result<Phone, &'static str> {
    if money < 0 {
        return Err("金额不正确");
    }
    Ok(Phone)
}

fn buy_phone1(money: i32, error_msg: &str) -> Result<Phone, &str> {
    if money < 0 {
        return Err(error_msg); // 返回调用者提供的引用
    }
    Ok(Phone)
}

fn status_code() -> Result<i32, String> {
    Err(String::from("not found"))
}

fn request_successed() -> Result<i32, String> {
    println!("请求成功");
    Ok(200)
}


fn request_failed(code: i32) -> Result<i32, String> {
    println!("请求失败 {}", code);
    Err(String::from("not found"))
}



// 