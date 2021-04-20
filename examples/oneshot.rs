//! 使用 tokio::sync::oneshot 创建一个新的一次性通道，用于异步任务之间发送单个值的通道。

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<i32>();
    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped")
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
