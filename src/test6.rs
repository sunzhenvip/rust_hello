use async_std::task;
use futures::executor::block_on;
use std::time::Duration;
async fn order() {
    println!("来一份大肠刺身");
    task::sleep(Duration::from_secs(10)).await;
}
fn main() {
    let future = order();
    block_on(future);
    println!("hello main")
}

fn main1() -> Result<i32, String> {
    Ok::<i32, String>(200)
}
