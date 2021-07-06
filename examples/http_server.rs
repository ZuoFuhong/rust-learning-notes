//! Multithreaded Web Server
//!
//! 目标：使用线程池异步响应的基本Web服务器，可以正常的关闭服务器，以清理池中的所有线程。
//! 启动流程：
//! 1.创建线程池
//! 2.创建 channel 生产、消费对象（在多线程中使用Arc计数Mutex引用，其中channel的receiver置于Mutex中）
//! 3.创建指定数量的worker线程持续消费channel中的任务。
//! 4.创建TCP服务端，主线程持续监听新连接。
//!
//! 请求处理流程：
//! 1.服务端主线程接受到客户端TCP连接
//! 2.创建一个闭包处理新连接
//! 3.主线程将闭包分配到线程池channel中（闭包使用Box分配堆内存）
//! 4.线程池中的worker线程持续从channel中获取闭包
//! 5.执行闭包，从TCP连接中读取请求数据，做出处理回包。
//!
//! 停机流程：
//! 1.TCP服务端中断后，ThreadPool将在main末尾超出范围，Drop实现将运行。
//! 2.ThreadPool的Drop方法第一次遍历worker线程组 发送terminate消息。
//! 3.ThreadPool的Drop方法第一次遍历worker线程组 执行worker线程join方法，直到线程结束。
//! 4.worker线程收到terminate消息后结束线程。
//! 5.当所有的worker线程都结束后，主线程也随之结束，最后进程结束。
//!
//! 线程模型：
//! -----------
//! - Client1 -------
//! -----------      \
//!                   \
//! -----------        \  -----------   dispatch    ---------------
//! - Client2 ----------> -  Accept - ------------> - Thread pool -
//! -----------        /  -----------               ---------------
//!                   /                                     |              -----------
//! -----------      /                                 push |           -> - worker1 -
//! - Client3 ------                                        |         /    -----------
//! -----------                                          --------   /
//!                                                      |      | /  pop   -----------
//!                                                      | chan |--------> - worker2 -
//!                                                      |      | \        -----------
//!                                                      --------  \
//!                                                                 \      -----------
//!                                                                  -->   - worker3 -
//!                                                                        -----------

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const HTML_PAGE: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <link rel=\"icon\" href=\"data:,\">
    <title>Rust HTTP Server</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buf.starts_with(get) {
        println!("Content-Length: {}", n);
        println!("{}", String::from_utf8_lossy(&buf[..n]));

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            HTML_PAGE.len(),
            HTML_PAGE
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(message) = receiver.lock().unwrap().recv() {
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");

        // 这里使用两个单独的循环，是为了防止死锁情况
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn http_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    // take方法是在Iterator特性中定义的，该迭代器产生前2个元素
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
    // ThreadPool将在main末尾超出范围，Drop实现将运行
}

fn main() {
    http_server();
}
