use std::time::Duration;
use tokio::runtime::Builder;


fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::time::sleep(Duration::from_secs(60)).await;
        println!("Hello, world!");
    });
    print!("主线程。。。。。。");
}

// 多线程初始化
fn test1() {
    let rt = Builder::new_multi_thread()
        .worker_threads(8)
        .thread_name("“hello")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();
    rt.block_on(async {
        tokio::time::sleep(Duration::from_secs(60)).await;
    });
    print!("主线程test。。。。。。");
}


// 单线程
fn test2() {
    let rt = Builder::new_current_thread().
        build().
        unwrap();
    rt.block_on(async {
        tokio::time::sleep(Duration::from_secs(60)).await;
    })
}