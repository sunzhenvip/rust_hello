fn assign<'a, 'b, T>(input: &'a mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}"); // 使用在被释放后的值 😿
}
