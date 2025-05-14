trait Phone {
    fn play_game(&self);
}

#[derive(Debug)]
struct Xiaomi;

impl Phone for Xiaomi {
    fn play_game(&self) {
        println!("使用小米玩游戏");
    }
}

trait Hello {
    fn hello(&self);
}

impl Hello for dyn Phone {
    fn hello(&self) {
        println!("为Phone trait object实现hello trait");
    }
}

fn main() {
    // let xm = Xiaomi;
    // xm.hello();
    let xm2: &dyn Phone = &Xiaomi;
    xm2.hello();

    xm2.play_game();

    let i32_total = add(66, 200);
    println!("i32 {}", i32_total);
    println!("f64 {}", add(66.0, 300f64));
}

use std::ops::Add;
/*fn add1<T>(x: T, y: T) -> T{
    x + y
}
*/

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn add2<T>(x: T, y: T) -> T
where
    T: Add<Output = T> + Add<Output = T>,
{
    x + y
}



fn add3<T: Add>(x: T, y: T) -> T::Output {
    x + y
}
