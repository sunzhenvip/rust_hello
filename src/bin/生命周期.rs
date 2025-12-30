fn identity(a: &i32) -> &i32 {
    a
}

fn example_1() {
    let x = 4;
    let x_ref = identity(&x);
    assert_eq!(*x_ref, 4);
}

fn example_2() {
    let mut x_ref: Option<&i32> = None;
    {
        let x = 7;
        x_ref = Some(identity(&x));
    }
    // assert_eq!(*x_ref.unwrap(), 7);
}

fn main() {
    let mut x_ref: Option<&String> = None;

    x_ref.unwrap_or(&Default::default());
}

fn print3<'a>(s: &'a str, b: &'a str, c: &'a str) -> &'a str {
    unimplemented!()
}

fn substr2(s: &str, until: usize) -> &str {
    s
}

fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}

struct S<'r> {
    x: &'r [u8],
}

impl<'a> S<'a> {
    fn get_x(&'a self) -> &'a [u8] {
        &self.x
    }
}

fn print2(s: &'_ str) {}

fn foo<'a>(input: &'a str) -> S<'a> {
    unimplemented!()
}

fn example_a<'a, 'b, 'c, 'd>(a: &'a i32, b: &'b i32, d: &'c Option<&'d i32>) {
    // d 要比 c 长，因为 c 的引用 用了d的引用
    // 所以c就一定比d短 或者说 d 就一定比c长
    unimplemented!()
}
