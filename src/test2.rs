use std::ops::Add;

trait Phone {
    type Cpu; // 关联类型

    fn new(brand: String, cpu: Self::Cpu) -> Self;
}

#[derive(Debug)]
struct Huawei {
    brand: String,
    cpu: String,
}

impl Phone for Huawei {
    type Cpu = String; // 指定关联了什么类型

    fn new(brand: String, cpu: Self::Cpu) -> Self {
        Huawei { brand, cpu }
    }
}

trait Hello {
    fn hello(&self);
}

impl Hello for i32 {
    fn hello(&self) {
        println!("hello {:?}", self);
    }
}

#[derive(Debug)]
struct Xiaomi {
    price: u16,
}
impl Add for Xiaomi {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        println!("{:?}", self);

        println!("{:?}", rhs);

        self.price + rhs.price
    }
}

#[derive(Debug)]
struct Xiaomi2 {
    price: u16,
}


impl Add for Xiaomi2 {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        println!("{:?}", self);
        println!("{:?}", rhs);

        self.price + rhs.price
    }
}

fn main() {
    let hw = Huawei::new("华为".to_string(), "麒麟9000s".to_string());

    println!("{:?}", hw);

    // let cc = C;
    // <C as B>::play_game(&cc);
    // <C as B>::play_game(&cc);

    let n = 200;
    n.hello();
    300.hello();

    let xm1 = Xiaomi2 { price: 10 };
    let xm2 = Xiaomi2 { price: 20 };

    let total = xm1 + xm2;

    println!("xm1 + xm2 = {}", total);

    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("failed");


    println!("{}", ok_result.custom_method());  // 输出: "Success: 42"
    println!("{}", err_result.custom_method()); // 输出: "Error: failed"
}


// 1. 定义你的自定义 trait
trait ResultExt<T, E> {
    fn custom_method(&self) -> String;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    T: std::fmt::Display, // 可选：添加约束
    E: std::fmt::Display, // 可选：添加约束
{
    fn custom_method(&self) -> String {
        match self {
            Ok(val) => format!("Success: {}", val),
            Err(err) => format!("Error: {}", err),
        }
    }
}


