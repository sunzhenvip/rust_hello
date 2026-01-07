// use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

// 参考资料 https://rustwiki.org/zh-CN/reference/lifetime-elision.html#默认的-trait对象的生命周期
fn main() {
    println!("asda");
}

trait Foo {
    fn foo(&self) -> i32 {
        println!("saa");
        10
    }
}

#[derive(Debug)]
struct Abc {
    age: i32,
}

// #[async_trait]
trait Example24 {
    async fn fetch(trace_id: &str, span_id: &str);
    // {
    //     todo!()
    // }
}

// fn fetch<'a, 'b>(trace_id: &'a str, span_id: &'b str) -> Box<dyn Future<Output = ()> + 'static> {
fn fetch<'a, 'b: 'a>(trace_id: &'a str, span_id: &'b str) -> Box<dyn Future<Output = ()> + 'a> {
    Box::new(async move {
        println!("{} {}", trace_id, span_id);
        println!("{} {}", trace_id, span_id);
    })
}

fn fetch2<'a, 'b, 'c>(trace_id: &'a str, span_id: &'b str) -> Box<dyn Future<Output = ()> + 'c>
where
    'a: 'c,
    'b: 'c,
{
    Box::new(async move {
        println!("{} {}", trace_id, span_id);
        println!("{} {}", trace_id, span_id);
    })
}
